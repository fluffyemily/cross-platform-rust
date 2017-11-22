#ifndef categories_h
#define categories_h

#import "items.h"

struct list_manager;
struct label;
struct item;

const struct label** get_all_labels(const struct list_manager* manager);
const void label_manager_create_item(const struct list_manager* manager, const struct item *item);
const void label_manager_update_item(const struct list_manager* manager, const struct item *item);

const size_t label_list_count(const struct label** list);
const void label_list_destroy(const struct label** list);
const struct label* label_list_item_at(const struct label** list, size_t index);
const void add_label(const struct label** list, const struct label* label);

struct label* label_new(const struct list_manager* manager, const char* name);
const void label_destroy(const struct label* label);
const char* label_get_name(const struct label* label);
const char** label_get_color(const struct label* label);
const void label_set_color(struct label* _Nonnull label, const char* _Nonnull color);


#endif /* categories_h */
