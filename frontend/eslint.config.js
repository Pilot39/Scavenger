const baseConfig = require('../eslint.config.base.js');

module.exports = {
  ...baseConfig,
  env: {
    browser: true,
    node: true,
    jest: true,
  },
  parserOptions: {
    project: './tsconfig.json',
    tsconfigRootDir: __dirname,
  },
  settings: {
    ...baseConfig.settings,
    react: {
      version: 'detect',
    },
  },
  rules: {
    ...baseConfig.rules,
    // Frontend-specific rules
    'react/no-unescaped-entities': 'off',
    '@typescript-eslint/no-explicit-any': 'warn',
  },
};
