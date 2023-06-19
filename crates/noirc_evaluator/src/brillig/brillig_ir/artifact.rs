use crate::brillig::brillig_ir::ReservedRegisters;
use acvm::acir::brillig_vm::{
    BinaryFieldOp, BinaryIntOp, Opcode as BrilligOpcode, RegisterIndex, Value,
};
use indexmap::IndexSet;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, Clone)]
/// Artifacts resulting from the compilation of a function into brillig byte code.
/// Currently it is just the brillig bytecode of the function.
pub(crate) struct BrilligArtifact {
    pub(crate) byte_code: Vec<BrilligOpcode>,
    /// The set of jumps that need to have their locations
    /// resolved.
    unresolved_jumps_and_calls: Vec<(JumpInstructionPosition, UnresolvedJumpLocation)>,
    /// A map of labels to their position in byte code.
    labels: HashMap<Label, OpcodeLocation>,
    /// Set of labels which are external to the bytecode.
    ///
    /// This will most commonly contain the labels of functions
    /// which are defined in other bytecode, that this bytecode has called.
    /// TODO: perhaps we should combine this with the `unresolved_jumps` field
    /// TODO: and have an enum which indicates whether the jump is internal or external
    // unresolved_external_call_labels: Vec<(JumpInstructionPosition, UnresolvedJumpLocation)>,
    /// Labels that need to be imported from other bytecode, since they are called by this bytecode.
    external_labels: IndexSet<Label>,
    /// The number of return values that this function will return.
    number_of_return_parameters: usize,

    /// The number of arguments that this function will take.
    number_of_arguments: usize,
}

/// A pointer to a location in the opcode.
pub(crate) type OpcodeLocation = usize;
/// An identifier for a location in the code.
///
/// It is assumed that an entity will keep a map
/// of labels to Opcode locations.
pub(crate) type Label = String;
/// Pointer to a unresolved Jump instruction in
/// the bytecode.
pub(crate) type JumpInstructionPosition = OpcodeLocation;

/// When constructing the bytecode, there may be instructions
/// which require one to jump to a specific region of code (function)
///
/// The position of a function cannot always be known
/// at this point in time, so Jumps are unresolved
/// until all functions/all of the bytecode has been processed.
/// `Label` is used as the jump location and once all of the bytecode
/// has been processed, the jumps are resolved using a map from Labels
/// to their position in the bytecode.
pub(crate) type UnresolvedJumpLocation = Label;

impl BrilligArtifact {
    /// Initialize an artifact with the number of arguments and return parameters
    pub(crate) fn new(
        number_of_arguments: usize,
        number_of_return_parameters: usize,
    ) -> BrilligArtifact {
        BrilligArtifact {
            byte_code: Vec::new(),
            unresolved_jumps_and_calls: Vec::new(),
            labels: HashMap::new(),
            number_of_return_parameters,
            number_of_arguments,
            external_labels: IndexSet::new(),
        }
    }

    /// Links Brillig artifact and resolve all unresolved jump instructions.
    ///
    /// Current usage of this method, does not link two independent Brillig artifacts.
    /// `Self` at this point in time
    ///
    /// TODO: This method could be renamed to `link_and_resolve_jumps`
    /// TODO: We could make this consume self, so the Clone is explicitly
    /// TODO: done by the caller
    pub(crate) fn link(
        artifact_to_append: &BrilligArtifact,
        dependency_map: &HashMap<Label, BrilligArtifact>,
    ) -> Vec<BrilligOpcode> {
        let mut linked_artifact = BrilligArtifact::default();

        linked_artifact.entry_point_instruction(artifact_to_append.number_of_arguments);
        // First we append the artifact to the end of the current artifact
        // Updating the offsets of the appended artefact, so that the jumps
        // are still correct.
        linked_artifact.append_artifact(artifact_to_append);

        // Resolve external dependencies/labels for this linked artifact
        // If those dependencies have linked artifacts that need to be resolved
        // then this process is done recursively
        let mut resolved = HashSet::new();
        linked_artifact.exit_point_instruction(artifact_to_append.number_of_return_parameters);
        linked_artifact.resolve_external_dependencies(&mut resolved, dependency_map);

        linked_artifact.resolve_jumps();

        linked_artifact.byte_code.clone()
    }
    /// Resolve external labels in this artifact.
    ///
    /// TODO: Explain how this deals with circular dependencies of artifacts
    /// TODO and or how we avoid this/ perhaps this is an invalid artifact that should
    /// TODO never happen, even when a function calls itself, or when there is a circular dependency
    /// TODO between two/three functions
    ///
    fn resolve_external_dependencies(
        &mut self,
        // labels to resolve
        //
        // This ensures that we do not append unused artifacts to our assembly
        //
        // Note: We do not use a HashSet because we want to preserve the order
        // in which the labels are resolved. This is important because this means
        // we get the same bytecode output on different runs.
        // to_resolve: &mut Vec<Label>,
        // resolved external labels
        // This ensures that once we've resolved a label by
        // appending the artifact to the current linked artifact
        // we do not append it again.
        resolved: &mut HashSet<Label>,
        // dependency map of all compiled brillig artifacts
        dependency_map: &HashMap<Label, BrilligArtifact>,
    ) {
        dbg!(&self.external_labels);
        loop {
            let external_label_to_resolve = match self.external_labels.pop() {
                Some(label_to_resolve) => label_to_resolve,
                None => {
                    break;
                }
            };

            // If the label has already been resolved, then we do not need to do anything
            if resolved.contains(&external_label_to_resolve) {
                continue;
            }

            // If the label is not in the dependency map, then we have a panic
            // as the label is not defined in any of the artifacts.

            if !dependency_map.contains_key(&external_label_to_resolve) {
                unreachable!(
                    "the label {external_label_to_resolve} is not defined in any of the artifacts"
                );
            }

            // If the label is in the dependency map, then we need to resolve it
            // by appending the artifact to the current linked artifact
            // Then adding this artifact's external labels to the list of labels to resolve
            let artifact_to_append = dependency_map.get(&external_label_to_resolve).unwrap();
            self.append_artifact(artifact_to_append);

            // TODO: Can we make this more elegant.
            self.byte_code.pop();
            self.push_opcode(BrilligOpcode::Return);

            // Add the labels of the artifact to the list of labels to resolve
            // This ensures that we resolve all external dependencies of external dependencies
            // self.external_labels.extend(artifact_to_append.external_labels.iter().cloned());

            // Add this artifact to the list of resolved artifacts and resolve its artifacts
            //
            // If we do not add it before resolving its dependencies, then if a dependency depends
            // on this appended artifact, it will append its artifact and we end up in an infinite loop
            resolved.insert(dbg!(external_label_to_resolve));
        }
    }

    /// Adds the instructions needed to handle entry point parameters
    ///
    /// And sets the starting value of the reserved registers
    pub(crate) fn entry_point_instruction(&mut self, num_arguments: usize) {
        // Translate the inputs by the reserved registers offset
        for i in (0..num_arguments).rev() {
            self.byte_code.push(BrilligOpcode::Mov {
                destination: ReservedRegisters::user_register_index(i),
                source: RegisterIndex::from(i),
            })
        }

        // Set the initial value of the stack pointer register
        self.byte_code.push(BrilligOpcode::Const {
            destination: ReservedRegisters::stack_pointer(),
            value: Value::from(0_usize),
        });
    }

    /// Adds the instructions needed to handle return parameters
    pub(crate) fn exit_point_instruction(&mut self, num_return_parameters: usize) {
        // We want all functions to follow the calling convention of returning
        // their results in the first `n` registers. So we modify the bytecode of the
        // function to move the return values to the first `n` registers once completed.
        //
        // Remove the ending stop
        // TODO: Shouldn't this be the case when we process a terminator instruction?
        // TODO: If so, then entry_point_instruction and exit_point_instruction should be
        // TODO put in brillig_gen.
        // TODO: entry_point is called when we process a function, and exit_point is called
        // TODO when we process a terminator instruction.
        let expected_stop = self.byte_code.pop().expect("expected at least one opcode");
        assert_eq!(expected_stop, BrilligOpcode::Stop, "expected a stop code");

        // TODO: this _seems_ like an abstraction leak, we need to know about the reserved
        // TODO: registers in order to do this.
        // Move the results to registers 0..n
        for i in 0..num_return_parameters {
            self.push_opcode(BrilligOpcode::Mov {
                destination: i.into(),
                source: ReservedRegisters::user_register_index(i),
            });
        }
        self.push_opcode(BrilligOpcode::Stop);
    }

    /// Link with an external brillig artifact.
    ///
    /// This method will offset the positions in the Brillig artifact to
    /// account for the fact that it is being appended to the end of this
    /// Brillig artifact (self).
    fn append_artifact(&mut self, obj: &BrilligArtifact) {
        let offset = self.index_of_next_opcode();
        for (jump_label, jump_location) in &obj.unresolved_jumps_and_calls {
            self.unresolved_jumps_and_calls.push((jump_label + offset, jump_location.clone()));
        }

        for (label_id, position_in_bytecode) in &obj.labels {
            let old_value = self.labels.insert(label_id.clone(), position_in_bytecode + offset);
            assert!(old_value.is_none(), "overwriting label {label_id} {old_value:?}");
        }

        self.external_labels.extend(obj.external_labels.iter().cloned());
        self.byte_code.extend_from_slice(&obj.byte_code);
    }

    /// Adds a brillig instruction to the brillig byte code
    pub(crate) fn push_opcode(&mut self, opcode: BrilligOpcode) {
        self.byte_code.push(opcode);
    }

    /// Adds a unresolved jump to be fixed at the end of bytecode processing.
    pub(crate) fn add_unresolved_jump(
        &mut self,
        jmp_instruction: BrilligOpcode,
        destination: UnresolvedJumpLocation,
    ) {
        assert!(
            Self::is_jmp_instruction(&jmp_instruction),
            "expected a jump instruction, but found {jmp_instruction:?}"
        );

        self.unresolved_jumps_and_calls.push((self.index_of_next_opcode(), destination));
        self.push_opcode(jmp_instruction);
    }
    /// Adds a unresolved external call that will be fixed once linking has been done.
    pub(crate) fn add_unresolved_external_call(
        &mut self,
        call_instruction: BrilligOpcode,
        destination: UnresolvedJumpLocation,
    ) {
        // TODO: Add a check to ensure that the opcode is a call instruction
        self.unresolved_jumps_and_calls.push((self.index_of_next_opcode(), destination.clone()));
        self.push_opcode(call_instruction);

        self.external_labels.insert(destination);
    }

    /// Returns true if the opcode is a jump instruction
    fn is_jmp_instruction(instruction: &BrilligOpcode) -> bool {
        matches!(
            instruction,
            BrilligOpcode::JumpIfNot { .. }
                | BrilligOpcode::JumpIf { .. }
                | BrilligOpcode::Jump { .. }
        )
    }

    /// Adds a label in the bytecode to specify where this block's
    /// opcodes will start.
    pub(crate) fn add_label_at_position(&mut self, label: String, position: OpcodeLocation) {
        let old_value = self.labels.insert(label.clone(), position);
        assert!(
            old_value.is_none(),
            "overwriting label {label}. old_value = {old_value:?}, new_value = {position}"
        );
    }

    /// Returns the index of the next opcode.
    ///
    /// This is useful for labelling regions of code
    /// before you have generated the opcodes for the region.
    pub(crate) fn index_of_next_opcode(&self) -> OpcodeLocation {
        self.byte_code.len()
    }

    /// Resolves all of the unresolved jumps in the program.
    ///
    /// Note: This should only be called once all blocks are processed and
    /// linkage with other bytecode has happened.
    fn resolve_jumps(&mut self) {
        for (location_of_jump, unresolved_location) in &self.unresolved_jumps_and_calls {
            let resolved_location = self.labels[dbg!(unresolved_location)];

            let jump_instruction = self.byte_code[*location_of_jump].clone();
            match jump_instruction {
                BrilligOpcode::Jump { location } => {
                    assert_eq!(location, 0, "location is not zero, which means that the jump label does not need resolving");

                    self.byte_code[*location_of_jump] =
                        BrilligOpcode::Jump { location: resolved_location };
                }
                BrilligOpcode::JumpIfNot { condition, location } => {
                    assert_eq!(location, 0, "location is not zero, which means that the jump label does not need resolving");

                    self.byte_code[*location_of_jump] =
                        BrilligOpcode::JumpIfNot { condition, location: resolved_location };
                }
                BrilligOpcode::JumpIf { condition, location } => {
                    assert_eq!(location, 0, "location is not zero, which means that the jump label does not need resolving");

                    self.byte_code[*location_of_jump] =
                        BrilligOpcode::JumpIf { condition, location: resolved_location };
                }
                BrilligOpcode::Call { location } => {
                    assert_eq!(
                        location, 0,
                        "location is not zero, which means that the label does not need resolving"
                    );
                    self.byte_code[*location_of_jump] =
                        BrilligOpcode::Call { location: resolved_location };
                }
                _ => unreachable!(
                    "all jump labels should point to a jump instruction in the bytecode"
                ),
            }
        }
    }
}
