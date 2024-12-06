import { render } from '@testing-library/svelte';
import Page from './+page.svelte';
import { describe, it, expect } from "vitest";

describe('Page', () => {

  it("it should render", () => {
    const { getByText } = render(Page, { props: { } });
    const element = getByText("Greet");
    expect(element).toBeInTheDocument();
  })
  
});