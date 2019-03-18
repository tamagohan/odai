package main

import (
	"testing"

	"github.com/k0kubun/pp"
)

func TestAddAndGet(t *testing.T) {
	c := New(10)
	got, hit := c.Get("a")
	if hit {
		t.Errorf("hit should be false")
	}
	if got != nil {
		t.Errorf("Value should be nil, because there is no key'a")
	}

	c.Add("a", 1)
	got, hit = c.Get("a")
	if !hit {
		t.Errorf("There has not to be key 'a'")
	}
	if got != 1 {
		t.Errorf("value should be 1, but got is %v", got)
	}

	c.Add("b", 2)
	got, _ = c.Get("b")
	if got != 2 {
		t.Errorf("value should be 2, but got is %v", got)
	}

	c.Add("a", 3)
	got, _ = c.Get("a")
	if got != 3 {
		t.Errorf("value should be 3, but got is %v", got)
	}
}

func TestRemove(t *testing.T) {
	c := New(10)
	// Even if cache doesn't has key 'a', c.Remove(a) should not panic
	c.Remove("a")

	c.Add("a", 1)
	c.Add("b", 2)
	c.Remove("a")
	if got := c.ddl.Len(); got != 1 {
		t.Errorf("Length of list should be 1, but got is %v.", got)
	}
	_, hit := c.Get("a")
	if hit {
		t.Errorf("There has not to be key 'a'")
	}
}

func TestInternalState(t *testing.T) {
	c := New(10)
	c.Add("a", 1)
	if got := c.ddl.Front().Value.(*entry).value; got != 1 {
		pp.Println(got)
		t.Errorf("First element of list should be value of key 'a', but got is %v", got)
	}
	if got := c.ddl.Len(); got != 1 {
		t.Errorf("Length of list should be 1, but got is %v.", got)
	}

	c.Add("b", 2)
	if got := c.ddl.Front().Value.(*entry).value; got != 2 {
		t.Errorf("First element of list should be value of key 'b', but got is %v.", got)
	}
	if got := c.ddl.Len(); got != 2 {
		t.Errorf("Length of list should be 2, but got is %v.", got)
	}

	c.Add("a", 1)
	if got := c.ddl.Front().Value.(*entry).value; got != 1 {
		t.Errorf("First element of list should be value of key 'a', but got is %v", got)
	}
	if got := c.ddl.Len(); got != 2 {
		t.Errorf("Length of list should be 2, but got is %v", got)
	}

	c.Get("b")
	if got := c.ddl.Front().Value.(*entry).value; got != 2 {
		t.Errorf("First element of list should be value of key 'b', but got is %v", got)
	}
}

func TestMaxEntries(t *testing.T) {
	c := New(2)
	c.Add("a", 1)
	c.Add("b", 2)
	if c.ddl.Len() != 2 {
		t.Errorf("Length of list should be 2")
	}

	c.Add("c", 3)
	if len := c.ddl.Len(); len != 2 {
		t.Errorf("c.ddl.Len() == %v", len)
	}
	_, hit := c.Get("a")
	if hit {
		t.Errorf("Cache should not contain key 'a'")
	}
}
