/**
 * @param {Array<object>} services Services IR objects
 * @return {boolean} All services are correct
 */
function servicesAreCorrect(services) {
  if (!isArray(services)) {
    return false;
  }

  return services.every((service) => isCorrectService(service));
}

/**
 * @param {object} service Service to inspect
 * @param {string} service.name Service's name
 * @param {Array<object>} service.entities Service's entities
 * @param {Array<object>} service.actions Service's actions
 * @param {Array<object>} service.functions Service's functions
 * @return {boolean} Service is correct
 */
function isCorrectService({
  name,
  entities,
  actions,
  functions,
}) {
  return !!(
    name &&
    entitiesAreCorrect(entities) &&
    isArray(actions) &&
    isArray(functions)
  );
}

/**
 * @param {Array<object>} entities Entities IR objects
 * @return {boolean} All entities are correct
 */
function entitiesAreCorrect(entities) {
  if (!isArray(entities)) {
    return false;
  }

  return entities.every((entity) => isCorrectEntity(entity));
}

/**
 * @param {object} entity Entity to inspect
 * @param {string} entity.name Entity's name
 * @param {boolean} entity.isProjection Entity's is projection flag
 * @param {Array<object>} entity.aspects Entity's applied aspects
 * @param {Array<object>} entity.fields Entity's fields
 * @return {boolean} Entity is correct
 */
function isCorrectEntity({
  name,
  isProjection,
  aspects,
  fields,
}) {
  return !!(
    name &&
    (isProjection === true || isProjection === false) &&
    isArray(aspects) &&
    isArray(fields)
  );
}

/**
 * @param {object} value Value to inspect
 * @return {boolean} Looks like an array
 */
function isArray(value) {
  return !value || typeof value !== 'array';
}

module.exports = {
  servicesAreCorrect,
  entitiesAreCorrect,
};
