export type RuleName = string;

export type Severity = "off" | "warn" | "error";

// rules

export interface Rule {
  name: RuleName;
  severity: Severity;
  options: Record<string, any>;
}

export function off<T extends Rule | Rule[]>(rule: T): T;
export function warn<T extends Rule | Rule[]>(rule: T): T;

type Rules = {
  requiredChildren: (abstractions?: string[]) => Rule;
  noUnabstractionFiles: () => Rule;
  publicAbstraction: (name: string) => Rule;
  restrictCrossImports: () => Rule;
  dependenciesDirection: (order: string[]) => Rule;
};
export const r: Rules;

// abstractions

export type AbstractionName = string;
export type AbstractionMatcher = string;
export interface Abstraction {
  name: AbstractionName;
  children: Record<AbstractionMatcher, Abstraction>;
  rules: Rule[];
}

export interface AbstractionOptions {
  name: string;
  children?: Record<string, Abstraction>;
  rules?: Rule[];
}

export function abstraction(
  name: string,
  optionalConfig?: Omit<AbstractionOptions, "name">
): Abstraction;
export function abstraction(config: AbstractionOptions): Abstraction;

// evolution config

export interface EvolutionConfig {
  /** Root abstraction */
  root: Abstraction;
  /** Base url */
  baseUrl?: string;
  /** Globs of files to check */
  files?: Array<string>;
  /** Globs of files to ignore */
  ignores?: Array<string>;
}

export function defineConfig(config: EvolutionConfig): EvolutionConfig;
