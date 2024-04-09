use noirc_errors::{Location, Span};
use noirc_frontend::{
    graph::CrateId,
    hir::{
        def_collector::dc_crate::{UnresolvedFunctions, UnresolvedTraitImpl},
        def_map::{LocalModuleId, ModuleId},
    },
    macros_api::{FileId, HirContext, MacroError},
    node_interner::FuncId,
    parse_program, FunctionReturnType, ItemVisibility, NoirFunction, UnresolvedTypeData,
};

use crate::utils::hir_utils::fetch_struct_trait_impls;

// Check if "compute_note_hash_and_nullifier(AztecAddress,Field,Field,Field,[Field; N]) -> [Field; 4]" is defined
fn check_for_compute_note_hash_and_nullifier_definition(
    functions_data: &[(LocalModuleId, FuncId, NoirFunction)],
    module_id: LocalModuleId,
) -> bool {
    functions_data.iter().filter(|func_data| func_data.0 == module_id).any(|func_data| {
        func_data.2.def.name.0.contents == "compute_note_hash_and_nullifier"
                && func_data.2.def.parameters.len() == 5
                && match &func_data.2.def.parameters[0].typ.typ {
                    UnresolvedTypeData::Named(path, _, _) => path.segments.last().unwrap().0.contents == "AztecAddress",
                    _ => false,
                }
                && func_data.2.def.parameters[1].typ.typ == UnresolvedTypeData::FieldElement
                && func_data.2.def.parameters[2].typ.typ == UnresolvedTypeData::FieldElement
                && func_data.2.def.parameters[3].typ.typ == UnresolvedTypeData::FieldElement
                // checks if the 5th parameter is an array and the Box<UnresolvedType> in
                // Array(Option<UnresolvedTypeExpression>, Box<UnresolvedType>) contains only fields
                && match &func_data.2.def.parameters[4].typ.typ {
                    UnresolvedTypeData::Array(_, inner_type) => {
                        matches!(inner_type.typ, UnresolvedTypeData::FieldElement)
                    },
                    _ => false,
                }
                // We check the return type the same way as we did the 5th parameter
                && match &func_data.2.def.return_type {
                    FunctionReturnType::Default(_) => false,
                    FunctionReturnType::Ty(unresolved_type) => {
                        match &unresolved_type.typ {
                            UnresolvedTypeData::Array(_, inner_type) => {
                                matches!(inner_type.typ, UnresolvedTypeData::FieldElement)
                            },
                            _ => false,
                        }
                    }
                }
    })
}

pub fn inject_compute_note_hash_and_nullifier(
    crate_id: &CrateId,
    context: &mut HirContext,
    unresolved_traits_impls: &[UnresolvedTraitImpl],
    collected_functions: &mut [UnresolvedFunctions],
) -> Result<(), (MacroError, FileId)> {
    // We first fetch modules in this crate which correspond to contracts, along with their file id.
    let contract_module_file_ids: Vec<(LocalModuleId, FileId)> = context
        .def_map(crate_id)
        .expect("ICE: Missing crate in def_map")
        .modules()
        .iter()
        .filter(|(_, module)| module.is_contract)
        .map(|(idx, module)| (LocalModuleId(idx), module.location.file))
        .collect();

    // If the current crate does not contain a contract module we simply skip it.
    if contract_module_file_ids.is_empty() {
        return Ok(());
    } else if contract_module_file_ids.len() != 1 {
        panic!("Found multiple contracts in the same crate");
    }

    let (module_id, file_id) = contract_module_file_ids[0];

    // If compute_note_hash_and_nullifier is already defined by the user, we skip auto-generation in order to provide an
    // escape hatch for this mechanism.
    // TODO(#4647): improve this diagnosis and error messaging.
    if collected_functions.iter().any(|coll_funcs_data| {
        check_for_compute_note_hash_and_nullifier_definition(&coll_funcs_data.functions, module_id)
    }) {
        return Ok(());
    }

    // In order to implement compute_note_hash_and_nullifier, we need to know all of the different note types the
    // contract might use. These are the types that implement the NoteInterface trait, which provides the
    // get_note_type_id function.
    let note_types = fetch_struct_trait_impls(context, unresolved_traits_impls, "NoteInterface");

    // We can now generate a version of compute_note_hash_and_nullifier tailored for the contract in this crate.
    let func = generate_compute_note_hash_and_nullifier(&note_types);

    // And inject the newly created function into the contract.

    // TODO(#4373): We don't have a reasonable location for the source code of this autogenerated function, so we simply
    // pass an empty span. This function should not produce errors anyway so this should not matter.
    let location = Location::new(Span::empty(0), file_id);

    // These are the same things the ModCollector does when collecting functions: we push the function to the
    // NodeInterner, declare it in the module (which checks for duplicate definitions), and finally add it to the list
    // on collected but unresolved functions.

    let func_id = context.def_interner.push_empty_fn();
    context.def_interner.push_function(
        func_id,
        &func.def,
        ModuleId { krate: *crate_id, local_id: module_id },
        location,
    );

    context.def_map_mut(crate_id).unwrap()
        .modules_mut()[module_id.0]
        .declare_function(
            func.name_ident().clone(), ItemVisibility::Public, func_id
        ).expect(
            "Failed to declare the autogenerated compute_note_hash_and_nullifier function, likely due to a duplicate definition. See https://github.com/AztecProtocol/aztec-packages/issues/4647."
        );

    collected_functions
        .iter_mut()
        .find(|fns| fns.file_id == file_id)
        .expect("ICE: no functions found in contract file")
        .push_fn(module_id, func_id, func.clone());

    Ok(())
}

fn generate_compute_note_hash_and_nullifier(note_types: &[String]) -> NoirFunction {
    let function_source = generate_compute_note_hash_and_nullifier_source(note_types);

    let (function_ast, errors) = parse_program(&function_source);
    if !errors.is_empty() {
        dbg!(errors.clone());
    }
    assert_eq!(errors.len(), 0, "Failed to parse Noir macro code. This is either a bug in the compiler or the Noir macro code");

    let mut function_ast = function_ast.into_sorted();
    function_ast.functions.remove(0)
}

fn generate_compute_note_hash_and_nullifier_source(note_types: &[String]) -> String {
    // TODO(#4649): The serialized_note parameter is a fixed-size array, but we don't know what length it should have.
    // For now we hardcode it to 20, which is the same as MAX_NOTE_FIELDS_LENGTH.

    if note_types.is_empty() {
        // Even if the contract does not include any notes, other parts of the stack expect for this function to exist,
        // so we include a dummy version.
        "
        unconstrained fn compute_note_hash_and_nullifier(
            contract_address: AztecAddress,
            nonce: Field,
            storage_slot: Field,
            note_type_id: Field,
            serialized_note: [Field; 20]
        ) -> pub [Field; 4] {
            assert(false, \"This contract does not use private notes\");
            [0, 0, 0, 0]
        }"
        .to_string()
    } else {
        // For contracts that include notes we do a simple if-else chain comparing note_type_id with the different
        // get_note_type_id of each of the note types.

        let if_statements: Vec<String> = note_types.iter().map(|note_type| format!(
            "if (note_type_id == {0}::get_note_type_id()) {{
                dep::aztec::note::utils::compute_note_hash_and_nullifier({0}::deserialize_content, note_header, serialized_note)
            }}"
        , note_type)).collect();

        let full_if_statement = if_statements.join(" else ")
            + "
            else {
                assert(false, \"Unknown note type ID\");
                [0, 0, 0, 0]
            }";

        format!(
            "
            unconstrained fn compute_note_hash_and_nullifier(
                contract_address: AztecAddress,
                nonce: Field,
                storage_slot: Field,
                note_type_id: Field,
                serialized_note: [Field; 20]
            ) -> pub [Field; 4] {{
                let note_header = dep::aztec::prelude::NoteHeader::new(contract_address, nonce, storage_slot);

                {}
            }}",
            full_if_statement
        )
    }
}