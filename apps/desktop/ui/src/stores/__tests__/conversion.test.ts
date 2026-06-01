import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useConversionStore } from '../conversion'
import { useSettingsStore } from '../settings'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

import { invoke } from '@tauri-apps/api/core'
const mockInvoke = vi.mocked(invoke)

const IMAGE_RESULT = {
  output_path: '/tmp/out/photo.webp',
  input_size: 200,
  output_size: 100,
  saved_bytes: 100,
}
const DOC_RESULT = {
  output_path: '/tmp/out/doc.html',
  input_size: 500,
  output_size: 800,
  saved_bytes: -300,
}
const AUDIO_RESULT = {
  output_path: '/tmp/out/track.mp3',
  input_size: 1_000_000,
  output_size: 150_000,
  saved_bytes: 850_000,
}

describe('useConversionStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  // ── addFiles ──────────────────────────────────────────────────────────────

  describe('addFiles', () => {
    it('adds image file with correct category and status', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'image')
      expect(store.queue).toHaveLength(1)
      expect(store.queue[0]).toMatchObject({
        name: 'photo.jpg',
        category: 'image',
        status: 'waiting',
      })
    })

    it('adds document file with correct category', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'doc.md', path: '/tmp/doc.md' }], 'document')
      expect(store.queue[0]).toMatchObject({ category: 'document', inputFormat: 'md' })
    })

    it('defaults category to image', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'photo.png', path: '/tmp/photo.png' }])
      expect(store.queue[0].category).toBe('image')
    })

    it('rejects unsupported extension', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'file.txt', path: '/tmp/file.txt' }], 'image')
      expect(store.queue).toHaveLength(0)
    })

    it('rejects document extension in image category', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'doc.md', path: '/tmp/doc.md' }], 'image')
      expect(store.queue).toHaveLength(0)
    })

    it('rejects image extension in document category', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'document')
      expect(store.queue).toHaveLength(0)
    })

    it('adds audio file with correct category', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'track.wav', path: '/tmp/track.wav' }], 'audio')
      expect(store.queue[0]).toMatchObject({ category: 'audio', inputFormat: 'wav' })
    })

    it('rejects audio extension in image category', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'track.mp3', path: '/tmp/track.mp3' }], 'image')
      expect(store.queue).toHaveLength(0)
    })

    it('rejects image extension in audio category', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'audio')
      expect(store.queue).toHaveLength(0)
    })

    it('does not add duplicate paths', () => {
      const store = useConversionStore()
      const file = { name: 'photo.jpg', path: '/tmp/photo.jpg' }
      store.addFiles([file], 'image')
      store.addFiles([file], 'image')
      expect(store.queue).toHaveLength(1)
    })

    it('adds multiple distinct files', () => {
      const store = useConversionStore()
      store.addFiles(
        [
          { name: 'a.jpg', path: '/tmp/a.jpg' },
          { name: 'b.png', path: '/tmp/b.png' },
        ],
        'image',
      )
      expect(store.queue).toHaveLength(2)
    })
  })

  // ── addDirectory ──────────────────────────────────────────────────────────

  describe('addDirectory', () => {
    it('calls list_directory recursively and filters by category', async () => {
      mockInvoke.mockResolvedValueOnce([
        '/tmp/dir/photo.jpg',
        '/tmp/dir/doc.md',
        '/tmp/dir/file.txt',
      ])
      const store = useConversionStore()
      await store.addDirectory('/tmp/dir', 'image')
      expect(mockInvoke).toHaveBeenCalledWith('list_directory', {
        dirPath: '/tmp/dir',
        recursive: true,
      })
      expect(store.queue).toHaveLength(1)
      expect(store.queue[0].name).toBe('photo.jpg')
    })

    it('filters document files in document category', async () => {
      mockInvoke.mockResolvedValueOnce([
        '/tmp/dir/photo.jpg',
        '/tmp/dir/doc.md',
        '/tmp/dir/article.html',
      ])
      const store = useConversionStore()
      await store.addDirectory('/tmp/dir', 'document')
      expect(store.queue).toHaveLength(2)
    })

    it('filters audio files in audio category', async () => {
      mockInvoke.mockResolvedValueOnce([
        '/tmp/dir/photo.jpg',
        '/tmp/dir/track.mp3',
        '/tmp/dir/album.flac',
        '/tmp/dir/doc.md',
      ])
      const store = useConversionStore()
      await store.addDirectory('/tmp/dir', 'audio')
      expect(store.queue).toHaveLength(2)
      expect(store.queue.every((f) => f.category === 'audio')).toBe(true)
    })
  })

  // ── removeFile ────────────────────────────────────────────────────────────

  describe('removeFile', () => {
    it('removes a file by id', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'image')
      const id = store.queue[0].id
      store.removeFile(id)
      expect(store.queue).toHaveLength(0)
    })

    it('is a no-op for unknown id', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'image')
      store.removeFile('nonexistent')
      expect(store.queue).toHaveLength(1)
    })
  })

  // ── retryFile ─────────────────────────────────────────────────────────────

  describe('retryFile', () => {
    it('resets error status to waiting and clears error message', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'image')
      store.queue[0].status = 'error'
      store.queue[0].error = 'some error'
      store.retryFile(store.queue[0].id)
      expect(store.queue[0].status).toBe('waiting')
      expect(store.queue[0].error).toBeUndefined()
    })

    it('does not reset non-error status', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'image')
      store.queue[0].status = 'done'
      store.retryFile(store.queue[0].id)
      expect(store.queue[0].status).toBe('done')
    })
  })

  // ── clearDone ─────────────────────────────────────────────────────────────

  describe('clearDone', () => {
    it('removes done items and keeps others', () => {
      const store = useConversionStore()
      store.addFiles(
        [
          { name: 'a.jpg', path: '/tmp/a.jpg' },
          { name: 'b.png', path: '/tmp/b.png' },
        ],
        'image',
      )
      store.queue[0].status = 'done'
      store.clearDone()
      expect(store.queue).toHaveLength(1)
      expect(store.queue[0].name).toBe('b.png')
    })

    it('is a no-op when nothing is done', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'a.jpg', path: '/tmp/a.jpg' }], 'image')
      store.clearDone()
      expect(store.queue).toHaveLength(1)
    })
  })

  // ── computed ──────────────────────────────────────────────────────────────

  describe('computed', () => {
    it('waiting returns only waiting items', () => {
      const store = useConversionStore()
      store.addFiles(
        [
          { name: 'a.jpg', path: '/tmp/a.jpg' },
          { name: 'b.png', path: '/tmp/b.png' },
        ],
        'image',
      )
      store.queue[0].status = 'done'
      expect(store.waiting).toHaveLength(1)
      expect(store.waiting[0].name).toBe('b.png')
    })

    it('done returns only done items', () => {
      const store = useConversionStore()
      store.addFiles([{ name: 'a.jpg', path: '/tmp/a.jpg' }], 'image')
      store.queue[0].status = 'done'
      expect(store.done).toHaveLength(1)
    })

    it('totalSaved sums savedBytes across all items', () => {
      const store = useConversionStore()
      store.addFiles(
        [
          { name: 'a.jpg', path: '/tmp/a.jpg' },
          { name: 'b.png', path: '/tmp/b.png' },
        ],
        'image',
      )
      store.queue[0].savedBytes = 100
      store.queue[1].savedBytes = 50
      expect(store.totalSaved).toBe(150)
    })
  })

  // ── convertAll ────────────────────────────────────────────────────────────

  describe('convertAll', () => {
    it('invokes convert_image for image category and marks file done', async () => {
      const settings = useSettingsStore()
      settings.outputFormat = 'webp'
      settings.outputDirectory = '/tmp/out'
      mockInvoke.mockResolvedValue(IMAGE_RESULT)

      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'image')
      await store.convertAll('image')

      expect(mockInvoke).toHaveBeenCalledWith(
        'convert_image',
        expect.objectContaining({ inputPath: '/tmp/photo.jpg', outputFormat: 'webp' }),
      )
      expect(store.queue[0].status).toBe('done')
      expect(store.queue[0].outputPath).toBe(IMAGE_RESULT.output_path)
    })

    it('invokes convert_document for document category', async () => {
      const settings = useSettingsStore()
      settings.outputFormat = 'html'
      settings.outputDirectory = '/tmp/out'
      mockInvoke.mockResolvedValue(DOC_RESULT)

      const store = useConversionStore()
      store.addFiles([{ name: 'doc.md', path: '/tmp/doc.md' }], 'document')
      await store.convertAll('document')

      expect(mockInvoke).toHaveBeenCalledWith(
        'convert_document',
        expect.objectContaining({ inputPath: '/tmp/doc.md', outputFormat: 'html' }),
      )
      expect(store.queue[0].status).toBe('done')
    })

    it('invokes convert_audio for audio category with bitrate', async () => {
      const settings = useSettingsStore()
      settings.outputFormat = 'mp3'
      settings.bitrate = 192
      settings.outputDirectory = '/tmp/out'
      mockInvoke.mockResolvedValue(AUDIO_RESULT)

      const store = useConversionStore()
      store.addFiles([{ name: 'track.wav', path: '/tmp/track.wav' }], 'audio')
      await store.convertAll('audio')

      expect(mockInvoke).toHaveBeenCalledWith(
        'convert_audio',
        expect.objectContaining({
          inputPath: '/tmp/track.wav',
          outputFormat: 'mp3',
          bitrate: 192,
        }),
      )
      expect(store.queue[0].status).toBe('done')
      expect(store.queue[0].outputPath).toBe(AUDIO_RESULT.output_path)
    })

    it('passes undefined bitrate for lossless audio formats', async () => {
      const settings = useSettingsStore()
      settings.outputFormat = 'flac'
      settings.bitrate = 320
      settings.outputDirectory = '/tmp/out'
      mockInvoke.mockResolvedValue(AUDIO_RESULT)

      const store = useConversionStore()
      store.addFiles([{ name: 'track.wav', path: '/tmp/track.wav' }], 'audio')
      await store.convertAll('audio')

      expect(mockInvoke).toHaveBeenCalledWith(
        'convert_audio',
        expect.objectContaining({ outputFormat: 'flac', bitrate: undefined }),
      )
    })

    it('sets status to error when invoke rejects', async () => {
      const settings = useSettingsStore()
      settings.outputFormat = 'webp'
      mockInvoke.mockRejectedValue('conversion failed')

      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'image')
      await store.convertAll('image')

      expect(store.queue[0].status).toBe('error')
      expect(store.queue[0].error).toBe('conversion failed')
    })

    it('skips files from the other category', async () => {
      const settings = useSettingsStore()
      settings.outputFormat = 'webp'
      mockInvoke.mockResolvedValue(IMAGE_RESULT)

      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'image')
      store.addFiles([{ name: 'doc.md', path: '/tmp/doc.md' }], 'document')
      await store.convertAll('image')

      expect(store.queue.find((f) => f.category === 'image')?.status).toBe('done')
      expect(store.queue.find((f) => f.category === 'document')?.status).toBe('waiting')
    })

    it('stops processing remaining files after cancel', async () => {
      const settings = useSettingsStore()
      settings.outputFormat = 'webp'

      const store = useConversionStore()
      store.addFiles(
        [
          { name: 'a.jpg', path: '/tmp/a.jpg' },
          { name: 'b.jpg', path: '/tmp/b.jpg' },
        ],
        'image',
      )

      mockInvoke
        .mockImplementationOnce(async () => {
          store.cancelConversion()
          return IMAGE_RESULT
        })
        .mockResolvedValue(IMAGE_RESULT)

      await store.convertAll('image')

      expect(store.queue[0].status).toBe('done')
      expect(store.queue[1].status).toBe('waiting')
    })

    it('does nothing when no waiting files', async () => {
      const store = useConversionStore()
      await store.convertAll('image')
      expect(mockInvoke).not.toHaveBeenCalled()
    })

    it('resets isConverting to false after completion', async () => {
      const settings = useSettingsStore()
      settings.outputFormat = 'webp'
      mockInvoke.mockResolvedValue(IMAGE_RESULT)

      const store = useConversionStore()
      store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'image')
      await store.convertAll('image')

      expect(store.isConverting).toBe(false)
    })
  })
})

describe('video category', () => {
  const VIDEO_RESULT = {
    output_path: '/tmp/out/clip.mkv',
    input_size: 50_000_000,
    output_size: 20_000_000,
    saved_bytes: 30_000_000,
  }

  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('accepts video extensions', () => {
    const store = useConversionStore()
    store.addFiles(
      [
        { name: 'clip.mp4', path: '/tmp/clip.mp4' },
        { name: 'movie.mkv', path: '/tmp/movie.mkv' },
        { name: 'screen.webm', path: '/tmp/screen.webm' },
      ],
      'video',
    )
    expect(store.queue).toHaveLength(3)
    expect(store.queue.every((f) => f.category === 'video')).toBe(true)
  })

  it('rejects non-video files', () => {
    const store = useConversionStore()
    store.addFiles([{ name: 'photo.jpg', path: '/tmp/photo.jpg' }], 'video')
    expect(store.queue).toHaveLength(0)
  })

  it('invokes convert_video with correct params', async () => {
    const settings = useSettingsStore()
    settings.outputFormat = 'mkv'
    settings.videoCodec = 'h265'
    mockInvoke.mockResolvedValue(VIDEO_RESULT)

    const store = useConversionStore()
    store.addFiles([{ name: 'clip.mp4', path: '/tmp/clip.mp4' }], 'video')
    await store.convertAll('video')

    expect(mockInvoke).toHaveBeenCalledWith(
      'convert_video',
      expect.objectContaining({ codec: 'h265', outputFormat: 'mkv' }),
    )
  })

  it('marks file as done after successful conversion', async () => {
    const settings = useSettingsStore()
    settings.outputFormat = 'mp4'
    settings.videoCodec = 'h264'
    mockInvoke.mockResolvedValue(VIDEO_RESULT)

    const store = useConversionStore()
    store.addFiles([{ name: 'clip.mkv', path: '/tmp/clip.mkv' }], 'video')
    await store.convertAll('video')

    expect(store.queue[0].status).toBe('done')
  })
})
