module.exports = {
  root: true,
  env: {
    browser: true,
    es2021: true
  },
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    ecmaFeatures: {
      jsx: true
    },
    project: './tsconfig.json'
  },
  plugins: ['@typescript-eslint', 'react-hooks', 'react-refresh', 'import'],
  extends: [
    'eslint:recommended', 
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/recommended-requiring-type-checking', 
    'plugin:react-hooks/recommended', 
    'plugin:import/recommended',
    'plugin:import/typescript',
    'prettier', 
    'plugin:storybook/recommended'
  ],
  ignorePatterns: ['dist', 'node_modules', '*.js'],
  rules: {
    'react-refresh/only-export-components': ['warn', { allowConstantExport: true }],
    
    // Import ordering rules
    'import/order': [
      'error',
      {
        groups: [
          'builtin',          // Node.js built-in modules
          'external',         // External libraries
          'internal',         // Internal modules (via paths/aliases)
          'parent',           // Parent directories  
          'sibling',          // Same directory
          'index',            // Index files
          'object',           // Object imports
          'type'              // Type imports
        ],
        pathGroups: [
          {
            pattern: 'react',
            group: 'external',
            position: 'before'
          },
          {
            pattern: 'react-*',
            group: 'external',
            position: 'before'
          },
          {
            pattern: '@/**',
            group: 'internal',
            position: 'before'
          },
          {
            pattern: '@scavngr/**',
            group: 'external',
            position: 'after'
          }
        ],
        pathGroupsExcludedImportTypes: ['react', 'type'],
        'newlines-between': 'always',
        alphabetize: {
          order: 'asc',
          caseInsensitive: true
        }
      }
    ],
    'import/newline-after-import': ['error', { count: 1 }],
    'import/no-duplicates': 'error',
    'import/no-unresolved': 'error',
    'import/consistent-type-specifier-style': ['error', 'prefer-top-level'],
    
    // Path and import restrictions
    'no-restricted-imports': [
      'error',
      {
        patterns: [
          {
            group: ['../**/src/**'],
            message: 'Use path aliases (@/) instead of relative imports crossing src boundaries'
          },
          {
            group: ['.**/types/index'],
            message: 'Import specific types instead of the entire types index'
          }
        ]
      }
    ],
    
    // Complexity and maintainability rules
    'max-lines': ['warn', { max: 500, skipBlankLines: true, skipComments: true }],
    'max-lines-per-function': ['warn', { max: 100, skipBlankLines: true, skipComments: true }],
    'complexity': ['warn', { max: 15 }],
    'max-depth': ['warn', { max: 4 }],
    'max-params': ['warn', { max: 5 }],
    
    // TypeScript specific rules
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
    '@typescript-eslint/explicit-function-return-type': 'off',
    '@typescript-eslint/explicit-module-boundary-types': 'off',
    '@typescript-eslint/no-explicit-any': 'warn',
    '@typescript-eslint/no-floating-promises': 'error',
    '@typescript-eslint/await-thenable': 'error'
  },
  settings: {
    'import/resolver': {
      typescript: {
        alwaysTryTypes: true,
        project: './tsconfig.json'
      },
      node: {
        extensions: ['.js', '.jsx', '.ts', '.tsx']
      }
    },
    react: {
      version: 'detect'
    }
  }
    'no-console': ['error', { allow: ['warn', 'error'] }]
  },
  overrides: [
    {
      files: ['*.stories.tsx', '*.stories.ts'],
      rules: {
        'no-console': 'off'
      }
    }
  ]
}
