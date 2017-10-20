#ifndef categories_h
#define categories_h

#import "items.h"

struct category;
struct item;

struct category** get_all_categories(const struct store* store);

const size_t category_list_count(const struct category** list);
const void category_list_destroy(const struct category** list);
const void add_category(const struct category** list, const struct category* category);

struct category* create_category(const struct store* store, const char* name);
struct category* category_new(const char* name);
const void category_destroy(const struct category* category);
const size_t category_get_id(const struct category* category);
const char* category_get_name(const struct category* category);
struct item** category_get_items(const struct category* category);
const size_t category_items_count(const struct category* category);


#endif /* categories_h */
