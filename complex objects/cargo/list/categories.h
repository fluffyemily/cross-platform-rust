#ifndef categories_h
#define categories_h

#import "items.h"

struct list_manager;
struct category;
struct item;

const struct category** get_all_categories(const struct list_manager* manager);
const void category_manager_create_item(const struct list_manager* manager, const struct item *item, size_t category_id);
const void category_manager_update_item(const struct list_manager* manager, const struct item *item);

const size_t category_list_count(const struct category** list);
const void category_list_destroy(const struct category** list);
const struct category* category_list_item_at(const struct category** list, size_t index);
const void add_category(const struct category** list, const struct category* category);

struct category* category_new(const struct list_manager* manager, const char* name);
const void category_destroy(const struct category* category);
const size_t category_get_id(const struct category* category);
const char* category_get_name(const struct category* category);
struct item** category_get_items(const struct category* category);
const size_t category_items_count(const struct category* category);
const struct item* category_item_at(const struct item** list, size_t index);
const void category_add_item(const struct category* category, const struct item* item);


#endif /* categories_h */
