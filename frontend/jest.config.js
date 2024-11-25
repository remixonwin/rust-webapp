module.exports = {
  // Performance optimizations
  maxWorkers: '50%', // Use half of available CPU cores
  testEnvironment: 'jsdom',
  
  // Test environment setup
  setupFilesAfterEnv: ['<rootDir>/src/tests/setup.js'],
  
  // Speed and performance configurations
  cache: true,
  clearMocks: true,
  
  // Test match patterns
  testMatch: [
    '<rootDir>/src/tests/**/*.test.js'
  ],
  
  // Coverage configuration
  collectCoverage: false,
  
  // Performance-focused test runner
  testRunner: 'jest-circus/runner',
  
  // Module and transform configurations
  transform: {
    '^.+\\.js$': 'babel-jest'
  },
  
  // Ignore specific paths
  testPathIgnorePatterns: [
    '/node_modules/',
    '/dist/'
  ],
  
  // Mocking configurations
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/src/$1',
    '\\.(css|less|scss|sass)$': 'identity-obj-proxy',
    '^.+\\.css$': 'identity-obj-proxy'
  },
  
  // Performance-related settings
  globals: {
    __DEV__: true
  }
};
