import type { Logger, LoggerOptions } from "pino";

export enum ServerEnvironment {
	DEV,
	PROD,
}

export type PinoLogger = Logger & {
	setLogLevel?: (NODE_ENV: ServerEnvironment) => LoggerOptions['level'];
}

