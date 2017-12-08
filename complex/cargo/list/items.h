#ifndef items_h
#define items_h

struct list_manager;
struct label;
struct item;

const struct item* _Nonnull list_manager_create_item(const struct list_manager* _Nonnull manager, const char* _Nonnull name, const int64_t* _Nullable due_date, const int64_t* _Nullable completion_date, struct label*_Nonnull* _Nonnull list);

const void list_manager_update_item(const struct list_manager* _Nonnull manager, const struct item* _Nonnull item, const char* _Nullable name, const int64_t* _Nullable due_date, const int64_t* _Nullable completion_date, struct label*_Nonnull* _Nullable list);

const struct item*_Nonnull*_Nonnull get_all_items(const struct list_manager* _Nonnull manager);
const size_t item_list_count(const struct item*_Nonnull* _Nonnull item);
const void item_list_destroy(const struct item*_Nonnull* _Nonnull item);
const struct item* _Nullable item_list_item_at(const struct item*_Nonnull* _Nonnull item, size_t index);

const void item_destroy(const struct item* _Nonnull item);

const char* _Nonnull item_get_uuid(const struct item* _Nonnull item);
const char* _Nonnull item_get_name(const struct item* _Nonnull item);
const void item_set_name(struct item* _Nonnull item, const char* _Nonnull description);
int64_t* _Nullable item_get_due_date(const struct item* _Nonnull item);
const void item_set_due_date(struct item* _Nonnull item, const int64_t* _Nullable due_date);
int64_t* _Nullable item_get_completion_date(const struct item* _Nonnull item);
const void item_set_completion_date(struct item* _Nonnull item, const int64_t* _Nullable completion_date);


const struct label*_Nonnull*_Nonnull item_get_labels(const struct list_manager* _Nonnull manager);
const size_t item_labels_count(const struct label*_Nonnull* _Nonnull label);
const struct label* _Nullable item_label_at(const struct label*_Nonnull* _Nonnull label, size_t index);

#endif /* items_h */
