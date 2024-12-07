import { getByText, render, waitFor  } from '@testing-library/svelte';
import View from './View.svelte';
import { describe, it, expect } from "vitest";

describe('View', () => {

    it("it should render", async () => {
        const text = "Submit";

        const { getByText } = render(View, { 
            
        });

        expect(getByText(text)).toBeInTheDocument();

    })
  
});