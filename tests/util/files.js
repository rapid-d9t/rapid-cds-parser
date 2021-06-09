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
};
