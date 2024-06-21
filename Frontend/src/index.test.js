import { describe, it, expect } from 'vitest';
import BiddingMarket from './routes/Main/BiddingMarket/+page.svelte';
import {removeelement2} from './routes/Main/BiddingMarket/+page.svelte';


describe('sum test', () => {
	it('adds 1 + 2 to equal 3', () => {
		expect(1 + 2).toBe(3);
	});
});

describe('Remove element test', () => {
	it('removes an element from an array of uids', () => {
	  const arrayofuids = [1, 2, 3, 4, 5];
	  const elementToRemove = 3;
	  const expectedArray = [1, 2, 4, 5];
	  const result = removeelement2(arrayofuids, elementToRemove);
	  expect(result).toEqual(expectedArray);
	});
});

