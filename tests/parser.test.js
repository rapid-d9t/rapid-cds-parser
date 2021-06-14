const parser = require('..');
const files = require('./util/files');

describe('Rapid CDS Parser Intergration Tests', () => {
  describe('With correct input', () => {
    test('generateIR successfully parses', async () => {
      await files.generateCorrectFile('test.cds');

      const ir = parser.generateIR('test.cds');

      await files.deleteFile('test.cds');

      expect(ir).toEqual(files.CORRECT_FILE_IR);
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
      await files.generateIncorrectFile('test.cds');

      expect(() => {
        parser.generateIR('test.cds');
      }).toThrow();

      await files.deleteFile('test.cds');
    });
  });
});
