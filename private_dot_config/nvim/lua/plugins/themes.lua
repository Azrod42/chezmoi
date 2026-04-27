return {
  { -- You can easily change to a different colorscheme.
    -- Change the name of the colorscheme plugin below, and then
    -- change the command in the config to whatever the name of that colorscheme is.
    --
    'folke/tokyonight.nvim',
    priority = 1000, -- Make sure to load this before all the other start plugins.
    lazy = false,
    config = function()
      ---@diagnostic disable-next-line: missing-fields
      require('tokyonight').setup {
        styles = {
          comments = { italic = false }, -- Disable italics in comments
        },
      }
      -- Load the colorscheme here.
      -- Like many other themes, this one has different styles, and you could load
      -- any other, such as 'tokyonight-storm', 'tokyonight-moon', or 'tokyonight-day'.
      vim.cmd.colorscheme 'tokyonight-night'
      vim.o.background = 'dark'
    end,
  },

  -- {
  --   'catppuccin/nvim',
  --   priority = 1000,
  --   lazy = false,
  --   config = function()
  --     require('catppuccin').setup {
  --       flavour = 'mocha', -- latte, frappe, macchiato, mocha
  --       background = { -- :h background
  --         light = 'latte',
  --         dark = 'mocha',
  --       },
  --       transparent_background = false, -- disables setting the background color.
  --       show_end_of_buffer = false, -- shows the '~' characters after the end of buffers
  --       term_colors = false, -- sets terminal colors (e.g. `g:terminal_color_0`)
  --       dim_inactive = {
  --         enabled = false, -- dims the background color of inactive window
  --         shade = 'dark',
  --         percentage = 0.15, -- percentage of the shade to apply to the inactive window
  --       },
  --       no_italic = false, -- Force no italic
  --       no_bold = false, -- Force no bold
  --       no_underline = false, -- Force no underline
  --       styles = { -- Handles the styles of general hi groups (see `:h highlight-args`):
  --         comments = { 'bold' }, -- Change the style of comments
  --         conditionals = {},
  --         loops = {},
  --         functions = {},
  --         keywords = {},
  --         strings = {},
  --         variables = {},
  --         numbers = {},
  --         booleans = {},
  --         properties = {},
  --         types = {},
  --         operators = {},
  --         -- miscs = {}, -- Uncomment to turn off hard-coded styles
  --       },
  --       color_overrides = {
  --         mocha = {
  --           -- overlay2 = '#6c7086',
  --           green = '#8bff82',
  --           blue = '#6baeff',
  --           yellow = '#ffea9b',
  --           mauve = '#ca9cff',
  --         },
  --       },
  --       custom_highlights = {},
  --       default_integrations = true,
  --       integrations = {
  --         cmp = true,
  --         blink_cmp = true,
  --         avante = true,
  --         gitsigns = true,
  --         noice = true,
  --         telescope = true,
  --         treesitter_context = true,
  --         dap = true,
  --         dap_ui = true,
  --         nvimtree = true,
  --         fzf = true,
  --         treesitter = true,
  --         notify = false,
  --         mini = {
  --           enabled = true,
  --           indentscope_color = '',
  --         },
  --       },
  --     }
  --     vim.cmd.colorscheme 'catppuccin'
  --     require('nvim-treesitter.configs').setup {
  --       highlight = { enable = false },
  --     }
  --
  --     vim.api.nvim_create_autocmd('ColorScheme', {
  --       callback = function()
  --         -- supprime le bg sur toutes les TS*
  --         vim.api.nvim_set_hl(0, 'TSAttribute', { bg = 'NONE' })
  --         vim.api.nvim_set_hl(0, 'TSString', { bg = 'NONE' })
  --         vim.api.nvim_set_hl(0, 'TSProperty', { bg = 'NONE' })
  --         -- etc. ajoutez les autres TS* qui vous gênent
  --       end,
  --     })
  --
  --     vim.o.background = 'dark'
  --   end,
  -- },
}
