import { describe, it, vi, expect, afterEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'

import BunproSectionVue from '../BunproSection.vue'
import LoadingIndicatorVue from '../LoadingIndicator.vue'

describe('BunproSection', () => {
  const fetchMock = vi.fn()
  global.fetch = fetchMock

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('displays a loading indicator', async () => {
    const mockResponse = {
      json: () => new Promise((resolve) => resolve({}))
    }
    fetchMock.mockResolvedValue(mockResponse)

    const wrapper = mount(BunproSectionVue)

    expect(wrapper.findComponent(LoadingIndicatorVue).exists()).toBe(true)
  })

  describe('when the request succeeds', () => {
    it('displays the information', async () => {
      const data = {
        data_updated_at: '2023-06-24T06:00:00Z',
        active_review_count: 8
      }
      const mockResponse = {
        status: 200,
        ok: true,
        json: () => new Promise((resolve) => resolve(data))
      }
      fetchMock.mockResolvedValue(mockResponse)

      const wrapper = mount(BunproSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain(8)
    })
  })

  describe('when the request fails', () => {
    it('displays an error message', async () => {
      const data = {
        message: 'uh oh'
      }
      const mockResponse = {
        status: 500,
        ok: false,
        json: () => new Promise((resolve) => resolve(data))
      }
      fetchMock.mockResolvedValue(mockResponse)

      const wrapper = mount(BunproSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain('uh oh')
    })
  })
})
