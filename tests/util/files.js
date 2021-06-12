const {promises: fs} = require('fs');

/**
 * Generates a correct cds file
 *
 * @param {string} fileName Path of a new correct cds file
 */
async function generateCorrectFile(fileName) {
  await fs.writeFile(
      fileName,
      `
        entity Test {
          field : TestType;
          field2    : TestType2;
        }
    
        service TestService {
          type test : Test;
    
          entity Test2 : Aspect1 {
            field3 : Test3;
          }
    
          action atest(arg1: Test);
          action atest1(arg1: Test) returns Test;
          action atest2(arg1: Test) returns array of Test;
    
          function ftest0() returns Test;
          function ftest1(arg1: Test) returns Test;
          function ftest2(arg1: Test) returns array of Test;
        }
        `,
  );
}

const CORRECT_FILE_IR = {
  types: [],
  entities: [
    {
      name: 'Test',
      aspects: [],
      isProjection: false,
      fields: [
        {
          name: 'field',
          hasDefault: false,
          type: 'TestType',
        },
        {
          name: 'field2',
          type: 'TestType2',
          hasDefault: false,
        },
      ],
    },
  ],
  services: [
    {
      actions: [
        {
          params: [
            {
              name: 'arg1',
              type: 'Test',
              isArrayed: false,
            },
          ],
          hasOutput: false,
          name: 'atest',
        },
        {
          hasOutput: true,
          params: [
            {
              name: 'arg1',
              isArrayed: false,
              type: 'Test',
            },
          ],
          name: 'atest1',
          output: {
            isArrayed: false,
            type: 'Test',
          },
        },
        {
          name: 'atest2',
          hasOutput: true,
          output: {
            type: 'Test',
            isArrayed: true,
          },
          params: [
            {
              type: 'Test',
              isArrayed: false,
              name: 'arg1',
            },
          ],
        },
      ],
      name: 'TestService',
      entities: [
        {
          isProjection: false,
          aspects: [
            'Aspect1',
          ],
          fields: [
            {
              hasDefault: false,
              name: 'field3',
              type: 'Test3',
            },
          ],
          name: 'Test2',
        },
      ],
      functions: [
        {
          output: {
            isArrayed: false,
            type: 'Test',
          },
          params: [],
          name: 'ftest0',
        },
        {
          output: {
            isArrayed: false,
            type: 'Test',
          },
          name: 'ftest1',
          params: [
            {
              isArrayed: false,
              type: 'Test',
              name: 'arg1',
            },
          ],
        },
        {
          name: 'ftest2',
          params: [
            {
              type: 'Test',
              isArrayed: false,
              name: 'arg1',
            },
          ],
          output: {
            type: 'Test',
            isArrayed: true,
          },
        },
      ],
    },
  ],
};

/**
 * Generates an incorrect cds file
 *
 * @param {string} fileName Path of a new incorrect cds file
 */
async function generateIncorrectFile(fileName) {
  await fs.writeFile(
      fileName,
      `
        entity Test {
          field : TestType;
          field2    : TestType2;
        }
    
        service TestService {
          type test : Test;
    
          entct1 {
            field3 : Test3;
          }
    
          action atest(arg1: Test);
          actrns Test;
          action atest2(arg1: Test) returns array of Test;
    
          function ftest0() returns Test;
          function ftest1(arg1: Test) returns Test;
          function ftest2();
        }
      `,
  );
}

/**
 * Deletes files by name
 *
 * @param {string} fileName File (name) to delete
 */
async function deleteFile(fileName) {
  await fs.unlink(fileName);
}

module.exports = {
  generateCorrectFile,
  generateIncorrectFile,
  deleteFile,
  CORRECT_FILE_IR,
};
