#include <stdint.h>
#include "logins.h"

struct store;

struct store* new_store(const char* uri);
void store_destroy(struct store* data);

struct login* create_login(const struct store* data, const char* username, const char* password);
const size_t validate_login(const struct store* store, const char* username, const char* password);

struct category** get_all_categories(const struct store* store);
struct category* create_category(const struct store* store, const char* name);
