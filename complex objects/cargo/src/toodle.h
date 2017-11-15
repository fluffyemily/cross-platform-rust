#include <stdint.h>
#include "logins.h"
#include "categories.h"

struct toodle;

struct toodle* new_toodle(const char* uri);
void toodle_destroy(struct toodle* toodle);

struct login_manager* toodle_logins(struct toodle* toodle);
struct list_manager* toodle_list(struct toodle* toodle);
