const parser = require('..');
const files = require('./util/files');

describe('Rapid CDS Parser Intergration Tests', () => {
  describe('With correct input', () => {
    test('generate_ir successfully parses', async () => {
      await files.generateCorrectFile('test.cds');

      const ir = parser.generate_ir('test.cds');

      await files.deleteFile('test.cds');

      expect(ir).toEqual(files.CORRECT_FILE_IR);
    });
  });

  describe('With unexisting file', () => {
    test('generate_ir drops with an error', () => {
      expect(() => {
        parser.generate_ir('test.cds');
      }).toThrow();
    });
  });

  describe('With syntactically incorrect input', () => {
    test('generate_ir drops with an error', async () => {
      await files.generateIncorrectFile('test.cds');

      expect(() => {
        parser.generate_ir('test.cds');
      }).toThrow();

      await files.deleteFile('test.cds');
    });
  });
});
