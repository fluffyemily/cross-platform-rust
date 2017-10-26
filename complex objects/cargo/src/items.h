#ifndef items_h
#define items_h

struct item;

struct item* item_new();
const void item_destroy(const struct item* item);


const size_t item_get_id(const struct item* item);
const char* item_get_description(const struct item* item);
const void item_set_description(struct item* item, const char* description);
const size_t item_get_created_at(const struct item* item);
const size_t item_get_due_date(const struct item* item);
const void item_set_due_date(struct item* item, size_t description);
const size_t item_get_is_complete(const struct item* item);
const void item_set_is_complete(struct item* item, size_t description);


#endif /* items_h */
