import { render, waitFor  } from '@testing-library/svelte';
import Page from './+page.svelte';
import { describe, it, expect } from "vitest";

vi.mock("web-lib", () => ({
  getRecords: vi.fn(),
}))

import { getRecords } from "web-lib";

describe('Page', () => {

  it("it should render", async () => {
    (getRecords as jest.Mock).mockResolvedValue(['Item 1', 'Item 2', 'Item 3']);
    const { container, getByText } = render(Page, { props: { } });

    await waitFor(() => {
      const listItems = container.querySelectorAll('li');
      expect(listItems).toHaveLength(3);
    });
    
    expect(getByText('Click Me')).toBeInTheDocument();

    expect(getRecords).toHaveBeenCalledWith();
  })
  
});