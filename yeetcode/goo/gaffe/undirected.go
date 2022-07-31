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

func (u *UndirectedGraph[T]) AddEdge(a *T, b *T) error {
	if missing := keyInMap(u.nodes, a, b); len(missing) > 0 {
		return fmt.Errorf("keys missing %+v", missing)
	}
	u.nodes[a] = append(u.nodes[a], b)
	u.nodes[b] = append(u.nodes[b], a)
	return nil
}

func (u *UndirectedGraph[T]) DFS(fn func(*T) bool) {
	var (
		visited = map[T]struct{}{}
		stack   []*T
	)
	for node := range u.nodes {
		if _, v := visited[*node]; v {
			continue
		}
		stack = append(stack, node)
		for len(stack) > 0 {
			elem := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			visited[*elem] = struct{}{}
			for _, edge := range u.nodes[elem] {
				if _, v := visited[*edge]; !v {
					stack = append(stack, edge)
				}
			}
			if !fn(elem) {
				return
			}
		}
	}
}

func (u *UndirectedGraph[T]) IsConnected() bool {
	var nodes []*T
	u.DFS(func(n *T) bool {
		nodes = append(nodes, n)
		return true
	})
	return len(nodes) == len(u.nodes)
}

func (u *UndirectedGraph[T]) Bridges() []struct{ a, b *T } {
	return nil
}

func keyInMap[T comparable, V any](m map[T]V, key ...T) (ret []T) {
	for _, k := range key {
		if _, ok := m[k]; !ok {
			ret = append(ret, k)
		}
	}
	return
}
