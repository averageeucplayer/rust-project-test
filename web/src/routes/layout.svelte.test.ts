import { render } from '@testing-library/svelte';
import Layout from './+layout.svelte';
import { describe, it } from "vitest";

describe('Layout', () => {

  it("it should render", () => {
    const { getByText } = render(Layout, { props: { } });
  })
  
});