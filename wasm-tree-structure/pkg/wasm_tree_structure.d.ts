/* tslint:disable */
/* eslint-disable */
/**
*/
export function greet(): void;
export class TreeStructure {
  free(): void;
/**
* @returns {TreeStructure} 
*/
  static new(): TreeStructure;
/**
* @param {string} new_node 
*/
  add_node(new_node: string): void;
/**
* @param {string} to_remove_node 
*/
  remove_node(to_remove_node: string): void;
/**
* @param {string} from_node 
* @param {string} to_node 
*/
  add_edge(from_node: string, to_node: string): void;
/**
* @param {string} from_node 
* @param {string} to_node 
*/
  remove_edge(from_node: string, to_node: string): void;
/**
* @param {string} check_node 
* @returns {any} 
*/
  get_neighbors(check_node: string): any;
/**
* @param {string} check_node 
* @returns {any} 
*/
  get_dfs(check_node: string): any;
}
