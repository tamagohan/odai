package main

import (
	"container/list"
	"fmt"
)

type Key interface{}
type entry struct {
	key   Key
	value interface{}
}

// Cache is a LRU Cache. It is not safe for conccurent access.
type Cache struct {
	// maxEntries is maximum number of key which cache can have.
	maxEntries int
	// ddl is double-linked list. It is used to get least recently used key effectively
	ddl   *list.List
	cache map[Key]*list.Element
}

// New creates a new cache.
func New(maxEntries int) *Cache {
	return &Cache{
		maxEntries: maxEntries,
		ddl:        list.New(),
		cache:      map[Key]*list.Element{},
	}
}

// Add adds a value to the cache.
func (c *Cache) Add(key Key, value interface{}) *Cache {
	if el, hit := c.cache[key]; hit {
		el.Value.(*entry).value = value
		c.ddl.MoveToFront(el)
		return c
	}

	if c.ddl.Len() >= c.maxEntries {
		lel := c.ddl.Back()
		c.remove(lel)
	}

	e := entry{key: key, value: value}
	el := c.ddl.PushFront(&e)
	c.cache[key] = el
	return c
}

// Get gets a value if cache has a given key.
func (c *Cache) Get(key Key) (value interface{}, ok bool) {
	if el, hit := c.cache[key]; hit {
		c.ddl.MoveToFront(el)
		return el.Value.(*entry).value, true
	}
	return
}

// Remove removes a given key from the cache.
func (c *Cache) Remove(key Key) {
	if el, hit := c.cache[key]; hit {
		c.remove(el)
		return
	}
	return
}

func (c *Cache) remove(el *list.Element) {
	delete(c.cache, el.Value.(*entry).key)
	c.ddl.Remove(el)
}

func main() {
	c := New(10)
	fmt.Println(c.Get("a"))
	c.Add("a", 1)
	fmt.Println(c.Get("a"))
	c.Add("b", 2)
	c.Add("a", 3)
}
