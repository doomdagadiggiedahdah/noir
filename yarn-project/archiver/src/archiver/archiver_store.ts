import {
  Body,
  ExtendedContractData,
  GetUnencryptedLogsResponse,
  L1ToL2Message,
  L2Block,
  L2BlockL2Logs,
  LogFilter,
  LogType,
  NewInboxLeaf,
  TxEffect,
  TxHash,
  TxReceipt,
} from '@aztec/circuit-types';
import { Fr } from '@aztec/circuits.js';
import { AztecAddress } from '@aztec/foundation/aztec-address';
import { ContractClassPublic, ContractInstanceWithAddress } from '@aztec/types/contracts';

/**
 * Represents the latest L1 block processed by the archiver for various objects in L2.
 */
export type ArchiverL1SynchPoint = {
  /** The last L1 block that added a new L2 block.  */
  addedBlock: bigint;
  /** The last L1 block that added messages from the new inbox. */
  // TODO(#4492): Clean this up and fix the naming
  newMessages: bigint;
  /** The last L1 block that added pending messages */
  addedMessages: bigint;
  /** The last L1 block that cancelled messages */
  cancelledMessages: bigint;
};

/**
 * Interface describing a data store to be used by the archiver to store all its relevant data
 * (blocks, encrypted logs, aztec contract data extended contract data).
 */
export interface ArchiverDataStore {
  /**
   * Append new blocks to the store's list.
   * @param blocks - The L2 blocks to be added to the store.
   * @returns True if the operation is successful.
   */
  addBlocks(blocks: L2Block[]): Promise<boolean>;

  /**
   * Append new block bodies to the store's list.
   * @param blockBodies - The L2 block bodies to be added to the store.
   * @returns True if the operation is successful.
   */
  addBlockBodies(blockBodies: Body[]): Promise<boolean>;

  /**
   * Gets block bodies that have the same txsEffectsHashes as we supply.
   *
   * @param txsEffectsHashes - A list of txsEffectsHashes.
   * @returns The requested L2 block bodies
   */
  getBlockBodies(txsEffectsHashes: Buffer[]): Promise<Body[]>;

  /**
   * Gets up to `limit` amount of L2 blocks starting from `from`.
   * @param from - Number of the first block to return (inclusive).
   * @param limit - The number of blocks to return.
   * @returns The requested L2 blocks.
   */
  getBlocks(from: number, limit: number): Promise<L2Block[]>;

  /**
   * Gets a tx effect.
   * @param txHash - The txHash of the tx corresponding to the tx effect.
   * @returns The requested tx effect (or undefined if not found).
   */
  getTxEffect(txHash: TxHash): Promise<TxEffect | undefined>;

  /**
   * Gets a receipt of a settled tx.
   * @param txHash - The hash of a tx we try to get the receipt for.
   * @returns The requested tx receipt (or undefined if not found).
   */
  getSettledTxReceipt(txHash: TxHash): Promise<TxReceipt | undefined>;

  /**
   * Append new logs to the store's list.
   * @param encryptedLogs - The encrypted logs to be added to the store.
   * @param unencryptedLogs - The unencrypted logs to be added to the store.
   * @param blockNumber - The block for which to add the logs.
   * @returns True if the operation is successful.
   */
  addLogs(
    encryptedLogs: L2BlockL2Logs | undefined,
    unencryptedLogs: L2BlockL2Logs | undefined,
    blockNumber: number,
  ): Promise<boolean>;

  /**
   * Append new L1 to L2 messages to the store.
   * @param messages - The L1 to L2 messages to be added to the store.
   * @param lastMessageL1BlockNumber - The L1 block number in which the last message was emitted.
   * @returns True if the operation is successful.
   */
  addNewL1ToL2Messages(messages: NewInboxLeaf[], lastMessageL1BlockNumber: bigint): Promise<boolean>;

  /**
   * Append new pending L1 to L2 messages to the store.
   * @param messages - The L1 to L2 messages to be added to the store.
   * @param l1BlockNumber - The block number of the L1 block that added the messages.
   * @returns True if the operation is successful.
   * TODO(#4492): Nuke the following when purging the old inbox
   */
  addPendingL1ToL2Messages(messages: L1ToL2Message[], l1BlockNumber: bigint): Promise<boolean>;

  /**
   * Remove pending L1 to L2 messages from the store (if they were cancelled).
   * @param entryKeys - The entry keys to be removed from the store.
   * @param l1BlockNumber - The block number of the L1 block that cancelled the messages.
   * @returns True if the operation is successful.
   * TODO(#4492): Nuke the following when purging the old inbox
   */
  cancelPendingL1ToL2EntryKeys(entryKeys: Fr[], l1BlockNumber: bigint): Promise<boolean>;

  /**
   * Messages that have been published in an L2 block are confirmed.
   * Add them to the confirmed store, also remove them from the pending store.
   * @param entryKeys - The entry keys to be removed from the store.
   * @returns True if the operation is successful.
   */
  confirmL1ToL2EntryKeys(entryKeys: Fr[]): Promise<boolean>;

  /**
   * Gets up to `limit` amount of pending L1 to L2 messages, sorted by fee
   * @param limit - The number of entries to return (by default NUMBER_OF_L1_L2_MESSAGES_PER_ROLLUP).
   * @returns The requested L1 to L2 entry keys.
   */
  getPendingL1ToL2EntryKeys(limit: number): Promise<Fr[]>;

  /**
   * Gets the confirmed L1 to L2 message corresponding to the given entry key.
   * @param entryKey - The entry key to look up.
   * @returns The requested L1 to L2 message or throws if not found.
   */
  getConfirmedL1ToL2Message(entryKey: Fr): Promise<L1ToL2Message>;

  /**
   * Gets new L1 to L2 message (to be) included in a given block.
   * @param blockNumber - L2 block number to get messages for.
   * @returns The L1 to L2 messages/leaves of the messages subtree (throws if not found).
   */
  getNewL1ToL2Messages(blockNumber: bigint): Promise<Fr[]>;

  /**
   * Gets up to `limit` amount of logs starting from `from`.
   * @param from - Number of the L2 block to which corresponds the first logs to be returned.
   * @param limit - The number of logs to return.
   * @param logType - Specifies whether to return encrypted or unencrypted logs.
   * @returns The requested logs.
   */
  getLogs(from: number, limit: number, logType: LogType): Promise<L2BlockL2Logs[]>;

  /**
   * Gets unencrypted logs based on the provided filter.
   * @param filter - The filter to apply to the logs.
   * @returns The requested logs.
   */
  getUnencryptedLogs(filter: LogFilter): Promise<GetUnencryptedLogsResponse>;

  /**
   * Add new extended contract data from an L2 block to the store's list.
   * @param data - List of contracts' data to be added.
   * @param blockNum - Number of the L2 block the contract data was deployed in.
   * @returns True if the operation is successful.
   */
  addExtendedContractData(data: ExtendedContractData[], blockNum: number): Promise<boolean>;

  /**
   * Get the extended contract data for this contract.
   * @param contractAddress - The contract data address.
   * @returns The extended contract data or undefined if not found.
   */
  getExtendedContractData(contractAddress: AztecAddress): Promise<ExtendedContractData | undefined>;

  /**
   * Gets the number of the latest L2 block processed.
   * @returns The number of the latest L2 block processed.
   */
  getBlockNumber(): Promise<number>;

  /**
   * Gets the last L1 block number processed by the archiver
   */
  getL1BlockNumber(): Promise<ArchiverL1SynchPoint>;

  /**
   * Add new contract classes from an L2 block to the store's list.
   * @param data - List of contract classes to be added.
   * @param blockNumber - Number of the L2 block the contracts were registered in.
   * @returns True if the operation is successful.
   */
  addContractClasses(data: ContractClassPublic[], blockNumber: number): Promise<boolean>;

  /**
   * Returns a contract class given its id, or undefined if not exists.
   * @param id - Id of the contract class.
   */
  getContractClass(id: Fr): Promise<ContractClassPublic | undefined>;

  /**
   * Add new contract instances from an L2 block to the store's list.
   * @param data - List of contract instances to be added.
   * @param blockNumber - Number of the L2 block the instances were deployed in.
   * @returns True if the operation is successful.
   */
  addContractInstances(data: ContractInstanceWithAddress[], blockNumber: number): Promise<boolean>;

  /**
   * Returns a contract instance given its address, or undefined if not exists.
   * @param address - Address of the contract.
   */
  getContractInstance(address: AztecAddress): Promise<ContractInstanceWithAddress | undefined>;

  /** Returns the list of all class ids known by the archiver. */
  getContractClassIds(): Promise<Fr[]>;
}