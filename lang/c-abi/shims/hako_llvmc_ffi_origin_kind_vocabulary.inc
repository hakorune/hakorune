/*
 * Generic pure lowering origin-kind vocabulary.
 *
 * Integer values are C-shim local ABI for the included lowering helpers.
 * Do not renumber without a dedicated migration.
 */

enum {
  ORG_NONE = 0,
  ORG_MAP_GET = 1,
  ORG_MAP_BIRTH = 2,
  ORG_ARRAY_BIRTH = 3,
  ORG_STRING = 4,
  ORG_ARRAY_STRING_BIRTH = 5,
  ORG_TYPED_OBJECT = 6,
  ORG_FILEBOX = 7
};
