package gaffe

import "fmt"

type UndirectedGraph[T comparable] struct {
	nodes map[*T][]*T
}

func NewUndirectedGraph[T comparable]() *UndirectedGraph[T] {
	return &UndirectedGraph[T]{nodes: map[*T][]*T{}}
}

func (u *UndirectedGraph[T]) AddNode(node *T) {
	u.nodes[node] = []*T{}
}

func keyInMap[T comparable, V any](m map[T]V, key ...T) (ret []T) {
	for _, k := range key {
		if _, ok := m[k]; !ok {
			ret = append(ret, k)
		}
	}
	return
}

func (u *UndirectedGraph[T]) AddEdge(a *T, b *T) error {
	if missing := keyInMap(u.nodes, a, b); len(missing) > 0 {
		return fmt.Errorf("keys missing %+v", missing)
	}
	u.nodes[a] = append(u.nodes[a], b)
	u.nodes[b] = append(u.nodes[b], a)
	return nil
}

// connectivity

// bridges
