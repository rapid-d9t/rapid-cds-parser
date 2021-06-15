const parser = require('..');
const TestFromFileExecutor = require('./util/files/TestFromFileExecutor');

describe('Rapid CDS Parser Intergration Tests', () => {
  describe('With correct input', () => {
    test('generateIR successfully parses', async () => {
      const executor = new TestFromFileExecutor();

      await executor.executePositiveTests();
    });
  });

  describe('With unexisting file', () => {
    test('generateIR drops with an error', () => {
      expect(() => {
        parser.generateIR('test.cds');
      }).toThrow();
    });
  });

  describe('With syntactically incorrect input', () => {
    test('generateIR drops with an error', async () => {
      const executor = new TestFromFileExecutor();

      await executor.executeNegativeTests();
    });
  });
});
