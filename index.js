/**
 * JS wrapper around a rapid-cds-parser binary library
 * to make it usable in modern IDEs
 */
const lib = require('./lib');

/**
 * @typedef {object} Module
 * @property {Array<Service>} services
 * @property {Array<Entity>} entities
 * @property {Array<Type>} types
 */

/**
 * @typedef {object} Service
 * @property {string} name Service's name
 * @property {Array<Function>} functions Service's functions
 * @property {Array<Action>} actions Entity's actions
 * @property {Array<Entity>} entities Service's entities
 */

/**
 * @typedef {object} Function
 * @property {string} name Function's name
 * @property {Array<Param>} params Function's params
 * @property {Output} output Function's output
 */

/**
 * @typedef {object} Action
 * @property {string} name Action's name
 * @property {Array<Param>} params Action's params
 * @property {boolean} hasOutput Action has output
 * @property {undefined | Output} output Action's output
 */

/**
 * @typedef {object} Param
 * @property {string} name Param's name
 * @property {boolean} isArrayed Param's type is arrayed
 * @property {string} type Param's type
 */

/**
 * @typedef {object} Output
 * @property {boolean} isArrayed Output's type is arrayed
 * @property {string} type Output's type
 */

/**
 * @typedef {object} Entity
 * @property {string} name Entity's name
 * @property {Array<string>} aspects Applied aspects
 * @property {Array<Field>} fields Entity's fields
 * @property {boolean} isProjection Flag entity is projection to anouther entity
 * @property {undefined | string} isProjectionOn Name of projected entity
 */

/**
 * @typedef {object} Field
 * @property {string} name Fields's name
 * @property {string} type Fields's type
 * @property {boolean} hasDefault Field has default value
 * @property {*} defaultValue Field's default value
 */

/**
 * @typedef {object} Type
 * @property {string} name Type's name
 * @property {string} resolvesTo To which type this type resolves to
 */

/**
 * Generates IR representation of CDS definitions
 * @param {string} path Path to CDS index file
 * @return {Module} Generated IR
 */
function generateIR(path) {
  return lib.generateIR(path);
}

module.exports = {
  generateIR,
};
