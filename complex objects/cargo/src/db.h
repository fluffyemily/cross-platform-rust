#include <stdint.h>
#include "logins.h"
#include "categories.h"

struct store;

struct store* new_store(const char* uri);
void store_destroy(struct store* data);

struct login_manager* store_logins(struct store* store);
struct category_manager* store_categories(struct store* store);


