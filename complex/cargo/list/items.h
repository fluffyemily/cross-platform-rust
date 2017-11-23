#ifndef items_h
#define items_h

struct item;

const struct item* _Nonnull item_new();
const void item_destroy(const struct item* _Nonnull item);

const char* _Nonnull item_get_name(const struct item* _Nonnull item);
const void item_set_name(struct item* _Nonnull item, const char* _Nonnull description);
int64_t* _Nullable item_get_due_date(const struct item* _Nonnull item);
const void item_set_due_date(struct item* _Nonnull item, int64_t due_date);
int64_t* _Nullable item_get_completion_date(const struct item* _Nonnull item);
const void item_set_completion_date(struct item* _Nonnull item, int64_t completion_date);
struct label** item_get_labels(const struct item* item);
const size_t item_labels_count(const struct item* item);
const struct label* item_label_at(const struct label** list, size_t index);

#endif /* items_h */
