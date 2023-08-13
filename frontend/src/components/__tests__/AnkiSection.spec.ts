import { flushPromises, mount } from '@vue/test-utils'
import { afterEach, describe, vi, it, expect } from 'vitest'
import LoadingIndicatorVue from '../LoadingIndicator.vue'
import AnkiSectionVue, { type AnkiResponse } from '../AnkiSection.vue'
import UpdatedTimestampVue from '../UpdatedTimestamp.vue'

interface MockAnkiResponse extends Partial<AnkiResponse> {}

describe('AnkiSection', () => {
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
    function mockResponse(mockData: MockAnkiResponse) {
      const data = {
        data_updated_at: mockData.data_updated_at || '2023-06-24T06:00:00Z',
        active_review_count: mockData.active_review_count || 8,
        new_card_count: mockData.new_card_count || 14,
        total_new_card_count: mockData.total_new_card_count || 14
      }
      const mockResponse = {
        status: 200,
        ok: true,
        json: () => new Promise((resolve) => resolve(data))
      }
      fetchMock.mockResolvedValue(mockResponse)
    }

    it('displays the information', async () => {
      mockResponse({})

      const wrapper = mount(AnkiSectionVue)
      await flushPromises()

      expect(wrapper.text()).toContain('Current Reviews: 8')
      expect(wrapper.text()).toContain('New Cards: 14')
      expect(wrapper.findComponent(UpdatedTimestampVue).text()).toContain(
        'Data Fetched at: 24/6/23, 3:00 pm'
      )
    })

    describe('when the total new card count is below the daily limit', () => {
      it('does not show additional new cards', async () => {
        mockResponse({ new_card_count: 14, total_new_card_count: 14 })

        const wrapper = mount(AnkiSectionVue)
        await flushPromises()

        expect(wrapper.findAll('.super').length).toEqual(0)
      })
    })

    describe('when the total new card count is above the daily limit', () => {
      it('shows the extra cards', async () => {
        mockResponse({ new_card_count: 14, total_new_card_count: 114 })

        const wrapper = mount(AnkiSectionVue)
        await flushPromises()

        expect(wrapper.findAll('.super').length).toEqual(1)
        expect(wrapper.find('.super').text()).toEqual('(+ 100)')
      })
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
