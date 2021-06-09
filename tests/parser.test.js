const parser = require('..');
const files = require('./util/files');
const validations = require('./util/validations');

describe('Rapid CDS Parser Intergration Tests', () => {
  describe('With correct input', () => {
    test('generate_ir successfully parses', async () => {
      await files.generateCorrectFile('test.cds');

      const ir = parser.generate_ir('test.cds');

      expect(validations.servicesAreCorrect(ir.services)).toBeTruthy();
      expect(validations.entitiesAreCorrect(ir.entities)).toBeTruthy();
      await files.deleteFile('test.cds');
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