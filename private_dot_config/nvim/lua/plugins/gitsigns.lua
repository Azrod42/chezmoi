return {
  'lewis6991/gitsigns.nvim',
  opts = {
    signs = {
      add = { text = '+' },
      change = { text = '~' },
      delete = { text = '_' },
      topdelete = { text = '‾' },
      changedelete = { text = '~' },
    },

    on_attach = function(bufnr)
      local gs = require 'gitsigns'

      local function map(mode, l, r, opts)
        opts = opts or {}
        opts.buffer = bufnr
        vim.keymap.set(mode, l, r, opts)
      end
      -- Navigation
      map({ 'n', 'v' }, ']c', function()
        if vim.wo.diff then
          return ']c'
        end
        vim.schedule(function()
          gs.nav_hunk 'next'
        end)
        return '<Ignore>'
      end, { expr = true, desc = 'Jump to next git [c]hange' })
      map({ 'n', 'v' }, '[c', function()
        if vim.wo.diff then
          return '[c'
        end
        vim.schedule(function()
          gs.nav_hunk 'prev'
        end)
        return '<Ignore>'
      end, { expr = true, desc = 'Jump to previous git [c]hange' })
      -- Actions
      -- visual mode
      map('v', '<leader>gs', function()
        gs.stage_hunk { vim.fn.line '.', vim.fn.line 'v' }
      end, { desc = 'stage git hunk' })
      map('v', '<leader>gr', function()
        gs.reset_hunk { vim.fn.line '.', vim.fn.line 'v' }
      end, { desc = 'reset git hunk' })
      -- normal mode
      map('n', '<leader>gs', gs.stage_hunk, { desc = 'git [s]tage hunk' })
      map('n', '<leader>gr', gs.reset_hunk, { desc = 'git [r]eset hunk' })
      map('n', '<leader>gS', gs.stage_buffer, { desc = 'git [S]tage buffer' })
      map('n', '<leader>gu', gs.stage_hunk, { desc = 'git [u]ndo stage hunk' })
      map('n', '<leader>gR', gs.reset_buffer, { desc = 'git [R]eset buffer' })
      map('n', '<leader>gp', gs.preview_hunk, { desc = 'git [p]review hunk' })
      map('n', '<leader>gb', function()
        gs.blame_line { full = false }
      end, { desc = 'git [b]lame line' })
      map('n', '<leader>gd', gs.diffthis, { desc = 'git [d]iff against index' })
      map('n', '<leader>gD', function()
        gs.diffthis '@'
      end, { desc = 'git [D]iff against last commit' })
      -- Toggles
      map('n', '<leader>tb', gs.toggle_current_line_blame, { desc = '[T]oggle git show [b]lame line' })
      map('n', '<leader>td', gs.preview_hunk_inline, { desc = '[T]oggle git show [d]eleted' })
    end,
  },
}
