import { flushPromises, mount } from '@vue/test-utils'
import { afterEach, describe, vi, it, expect } from 'vitest'
import LoadingIndicatorVue from '../LoadingIndicator.vue'
import AnkiSectionVue from '../AnkiSection.vue'
import UpdatedTimestampVue from '../UpdatedTimestamp.vue'

describe('AnkiData', () => {
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

    const wrapper = mount(AnkiSectionVue)

    expect(wrapper.findComponent(LoadingIndicatorVue).exists()).toBe(true)
  })

  describe('when the request succeeds', () => {
    it('displays the information', async () => {
      const data = {
        data_updated_at: '2023-06-24T06:00:00Z',
        active_review_count: 8,
        new_card_count: 14
      }
      const mockResponse = {
        status: 200,
        ok: true,
        json: () => new Promise((resolve) => resolve(data))
      }
      fetchMock.mockResolvedValue(mockResponse)

      const wrapper = mount(AnkiSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain('Current Reviews: 8')
      expect(wrapper.text()).toContain('New Cards: 14')
      expect(wrapper.findComponent(UpdatedTimestampVue).text()).toContain(
        'Data Fetched at: 24/6/23, 3:00 pm'
      )
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

      const wrapper = mount(AnkiSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain('uh oh')
    })
  })
})
