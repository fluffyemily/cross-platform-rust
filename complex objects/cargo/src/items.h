#ifndef items_h
#define items_h

struct item;

const struct item* _Nonnull item_new();
const void item_destroy(const struct item* _Nonnull item);


size_t* _Nullable item_get_id(const struct item* _Nonnull item);
const char* _Nonnull item_get_description(const struct item* _Nonnull item);
const void item_set_description(struct item* _Nonnull item, const char* _Nonnull description);
const size_t item_get_created_at(const struct item* _Nonnull item);
int64_t* _Nullable item_get_due_date(const struct item* _Nonnull item);
const void item_set_due_date(struct item* _Nonnull item, int64_t due_date);
const size_t item_get_is_complete(const struct item* _Nonnull item);
const void item_set_is_complete(struct item* _Nonnull item, size_t is_complete);


#endif /* items_h */
