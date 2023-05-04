/** @type {import('ts-jest/dist/types').InitialOptionsTsJest} */
module.exports = {
  preset: "ts-jest",
  testEnvironment: "node",
  testTimeout: 60_000,
  verbose: true,
  rootDir: "./",
  modulePaths: ["<rootDir>/src", "<rootDir>/assets"],
  moduleDirectories: ["node_modules", "src"],
};
