package gaffe

import "fmt"

type (
	weightedEdge[T any] struct {
		elem   *T
		weight int
	}

	WeightedUndirectedGraph[T comparable] struct {
		nodes map[*T][]weightedEdge[T]
	}
)

func NewWeightedUndirectedGraph[T comparable]() *UndirectedGraph[T] {
	return &UndirectedGraph[T]{nodes: map[*T][]*T{}}
}

func (w *WeightedUndirectedGraph[T]) AddNode(node *T) {
	w.nodes[node] = []weightedEdge[T]{}
}

func (w *WeightedUndirectedGraph[T]) AddEdge(a *T, b *T, weight int) error {
	if missing := keyInMap(w.nodes, a, b); len(missing) > 0 {
		return fmt.Errorf("keys missing %+v", missing)
	}
	w.nodes[a] = append(w.nodes[a], weightedEdge[T]{elem: b, weight: weight})
	w.nodes[b] = append(w.nodes[b], weightedEdge[T]{elem: a, weight: weight})
	return nil
}

// minimum spanning tree
// djikstra
// a*
