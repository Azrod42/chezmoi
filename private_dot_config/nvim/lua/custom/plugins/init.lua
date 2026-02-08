-- You can add your own plugins here or in other files in this directory!
--  I promise not to create any merge conflicts in this directory :)
--
-- See the kickstart.nvim README for more information
return {
  -- waka-time
  { 'wakatime/vim-wakatime', lazy = false },

  -- multi line
  { 'mg979/vim-visual-multi', lazy = false },

  { 'ziglang/zig.vim' },

  {
    'vuki656/package-info.nvim',
    ft = 'json',
    dependencies = { 'MunifTanjim/nui.nvim' },
    config = function()
      require('package-info').setup {
        hide_up_to_date = true,
      }
    end,
  },

  {
    'zbirenbaum/copilot.lua',
    config = function()
      require('copilot').setup {
        panel = {
          enabled = true,
          auto_refresh = false,
          keymap = {
            jump_prev = '[[',
            jump_next = ']]',
            accept = '<CR>',
            refresh = 'gr',
            open = '<M-CR>',
          },
          layout = {
            position = 'bottom', -- | top | left | right | horizontal | vertical
            ratio = 0.4,
          },
        },
        suggestion = {
          enabled = true,
          auto_trigger = true,
          hide_during_completion = true,
          debounce = 75,
          trigger_on_accept = true,
          keymap = {
            accept = '<Tab>',
            accept_word = false,
            accept_line = false,
            next = '<M-n>',
            prev = '<M-p>',
            dismiss = '<C-]>',
          },
        },
        filetypes = {
          yaml = false,
          markdown = false,
          help = false,
          gitcommit = false,
          gitrebase = false,
          hgcommit = false,
          svn = false,
          cvs = false,
          ['.'] = false,
        },
        copilot_node_command = 'node', -- Node.js version must be > 20
        workspace_folders = {},
        copilot_model = 'copilot/GPT-5.1-Codex',
        root_dir = function()
          return vim.fs.dirname(vim.fs.find('.git', { upward = true })[1])
        end,
      }
    end,
  },

  -- {
  --   'github/copilot.vim',
  --   config = function()
  --     require('copilot').setup {
  --       suggestion = { enabled = false },
  --       panel = { enabled = false },
  --     }
  --     vim.keymap.set({ 'n' }, '<leader>cd', '<cmd>Copilot disable<CR>', { desc = '[C]opilot [D]isable' })
  --     vim.keymap.set({ 'n' }, '<leader>ce', '<cmd>Copilot enable<CR>', { desc = '[C]opilot [E]nable' })
  --   end,
  -- },
  {
    'folke/lazydev.nvim',
    ft = 'lua',
    cmd = 'LazyDev',
    opts = {
      dependencies = {
        -- Manage libuv types with lazy. Plugin will never be loaded
        { 'Bilal2453/luvit-meta', lazy = true },
      },
      library = {
        { path = '~/Documents/avante.nvim/lua', words = { 'avante' } },
        { path = 'luvit-meta/library', words = { 'vim%.uv' } },
      },
    },
  },
  -- {
  --   'zbirenbaum/copilot-cmp',
  --   config = function()
  --     require('copilot_cmp').setup()
  --   end,
  -- },

  -- {
  --   'sindrets/diffview.nvim',
  --   config = function()
  --     vim.keymap.set({ 'n' }, '<leader>dm', '<cmd>DiffviewOpen<CR>', { desc = '[D]iff [M]aster' })
  --     vim.keymap.set({ 'n' }, '<leader>dc', '<cmd>DiffviewClose<CR>', { desc = '[D]iff [C]lose' })
  --     vim.keymap.set({ 'n' }, '<leader>df', '<cmd>DiffviewFileHistory %<CR>', { desc = '[D]iff [F]ile' })
  --   end,
  -- },
}
