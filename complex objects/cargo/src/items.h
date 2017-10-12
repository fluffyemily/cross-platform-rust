#ifndef items_h
#define items_h

struct item;

struct item* item_new();
const void item_destroy(const struct item* item);

#endif /* items_h */
