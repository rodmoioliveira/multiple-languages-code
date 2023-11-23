package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

func main() {
	var val atomic.Uint64
	var wg sync.WaitGroup

	for i := 0; i < 1000; i++ {
		wg.Add(1)

		go func() {
			val.Add(1)
			fmt.Println("Result: ", val.Load())

			wg.Done()
		}()
	}

	wg.Wait()
	fmt.Println("Result: ", val.Load())
}
