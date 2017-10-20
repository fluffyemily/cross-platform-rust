#include <stdint.h>
#include "logins.h"

struct store;

struct store* new_store(const char* uri);
void store_destroy(struct store* data);


