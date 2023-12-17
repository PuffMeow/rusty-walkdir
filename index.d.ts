import { IConfig } from './binding';

export type TWalkdirCallback = (path: string) => void;
export function walkdir(entry: string, callback: TWalkdirCallback): void;
export function walkdir(
  entry: string,
  config: IConfig,
  callback: TWalkdirCallback
): void;

export { IConfig };
