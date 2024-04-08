import { AbiType } from '@noir-lang/noirc_abi';
import { CompiledCircuit } from '@noir-lang/types';
import { PrimitiveTypesUsed, generateTsInterface, codegenStructDefinitions } from './noir_types.js';

// TODO: reenable this. See `abiTypeToTs` for reasoning.
// export type FixedLengthArray<T, L extends number> = L extends 0 ? never[]: T[] & { length: L };

const codegenPrelude = `/* Autogenerated file, do not edit! */

/* eslint-disable */

import { Noir, InputMap, CompiledCircuit, ForeignCallHandler } from "@noir-lang/noir_js"

export { ForeignCallHandler } from "@noir-lang/noir_js"
`;

const codegenFunction = (
  name: string,
  compiled_program: CompiledCircuit,
  function_signature: { inputs: [string, string][]; returnValue: string | null },
) => {
  const args = function_signature.inputs.map(([name]) => `${name}`).join(', ');
  const args_with_types = function_signature.inputs.map(([name, type]) => `${name}: ${type}`).join(', ');

  return `export const ${name}_circuit: CompiledCircuit = ${JSON.stringify(compiled_program)};

export async function ${name}(${args_with_types}, foreignCallHandler?: ForeignCallHandler): Promise<${
    function_signature.returnValue
  }> {
  const program = new Noir(${name}_circuit);
  const args: InputMap = { ${args} };
  const { returnValue } = await program.execute(args, foreignCallHandler);
  return returnValue as ${function_signature.returnValue};
}
`;
};

export const codegen = (programs: [string, CompiledCircuit][]): string => {
  let results = [codegenPrelude];
  const primitiveTypeMap = new Map<string, PrimitiveTypesUsed>();
  const structTypeMap = new Map<string, { name: string; type: AbiType }[]>();

  const functions: string[] = [];
  for (const [name, program] of programs) {
    const function_sig = generateTsInterface(program.abi, structTypeMap, primitiveTypeMap);
    functions.push(codegenFunction(name, stripUnwantedFields(program), function_sig));
  }

  const structTypeDefinitions: string = codegenStructDefinitions(structTypeMap, primitiveTypeMap);

  // Add the primitive Noir types that do not have a 1-1 mapping to TypeScript.
  const primitiveTypeAliases: string[] = [];
  for (const value of primitiveTypeMap.values()) {
    primitiveTypeAliases.push(`export type ${value.aliasName} = ${value.tsType};`);
  }

  results = results.concat(...primitiveTypeAliases, '', structTypeDefinitions, ...functions);

  return results.join('\n');
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function stripUnwantedFields(value: any): CompiledCircuit {
  const { abi, bytecode } = value;
  return { abi, bytecode };
}