import { describe, it, vi, expect, afterEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'

import WaniKaniVue from '../WaniKani.vue'
import LoadingIndicatorVue from '../LoadingIndicator.vue'

describe('WaniKani', () => {
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

    const wrapper = mount(WaniKaniVue)

    expect(wrapper.findComponent(LoadingIndicatorVue).exists()).toBe(true)
  })

  describe('when the request succeeds', () => {
    it('displays the information', async () => {
      const data = {
        data_updated_at: '2023-06-24T06:00:00Z',
        active_lesson_count: 66,
        active_review_count: 15
      }
      const mockResponse = {
        status: 200,
        json: () => new Promise((resolve) => resolve(data))
      }
      fetchMock.mockResolvedValue(mockResponse)

      const wrapper = mount(WaniKaniVue)
      await flushPromises()

      expect(wrapper.text()).toContain('66')
    })
  })

  describe('when the request fails', () => {
    it('displays an error message', async () => {
      const data = {
        message: 'uh oh'
      }
      const mockResponse = {
        status: 500,
        json: () => new Promise((resolve) => resolve(data))
      }
      fetchMock.mockResolvedValue(mockResponse)

      const wrapper = mount(WaniKaniVue)
      await flushPromises()

      expect(wrapper.text()).toContain('uh oh')
    })
  })
})
