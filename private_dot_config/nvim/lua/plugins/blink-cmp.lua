return { -- Autocompletion
  'saghen/blink.cmp',
  event = 'VimEnter',
  version = '1.*',
  dependencies = {
    -- Snippet Engine
    {
      'giuxtaposition/blink-cmp-copilot',
      'MahanRahmati/blink-nerdfont.nvim',
      'moyiz/blink-emoji.nvim',
      'rafamadriz/friendly-snippets',
    },
    {
      'L3MON4D3/LuaSnip',
      version = '2.*',
      build = (function()
        -- Build Step is needed for regex support in snippets.
        -- This step is not supported in many windows environments.
        -- Remove the below condition to re-enable on windows.
        if vim.fn.has 'win32' == 1 or vim.fn.executable 'make' == 0 then
          return
        end
        return 'make install_jsregexp'
      end)(),
      dependencies = {
        -- `friendly-snippets` contains a variety of premade snippets.
        --    See the README about individual language/framework/plugin snippets:
        --    https://github.com/rafamadriz/friendly-snippets
        -- {
        --   'rafamadriz/friendly-snippets',
        --   config = function()
        --     require('luasnip.loaders.from_vscode').lazy_load()
        --   end,
        -- },
      },
      opts = {},
    },
    'folke/lazydev.nvim',
  },
  --- @module 'blink.cmp'
  --- @type blink.cmp.Config
  opts = {
    keymap = {
      -- 'default' (recommended) for mappings similar to built-in completions
      --   <c-y> to accept ([y]es) the completion.
      --    This will auto-import if your LSP supports it.
      --    This will expand snippets if the LSP sent a snippet.
      -- 'super-tab' for tab to accept
      -- 'enter' for enter to accept
      -- 'none' for no mappings
      --
      -- For an understanding of why the 'default' preset is recommended,
      -- you will need to read `:help ins-completion`
      --
      -- No, but seriously. Please read `:help ins-completion`, it is really good!
      --
      -- All presets have the following mappings:
      -- <tab>/<s-tab>: move to right/left of your snippet expansion
      -- <c-space>: Open menu or open docs if already open
      -- <c-n>/<c-p> or <up>/<down>: Select next/previous item
      -- <c-e>: Hide menu
      -- <c-k>: Toggle signature help
      --
      -- See :h blink-cmp-config-keymap for defining your own keymap
      preset = 'default',

      -- For more advanced Luasnip keymaps (e.g. selecting choice nodes, expansion) see:
      --    https://github.com/L3MON4D3/LuaSnip?tab=readme-ov-file#keymaps
      sources = {
        default = { 'lsp', 'path', 'snippets', 'buffer', 'copilot' },
        providers = {
          copilot = {
            name = 'copilot',
            module = 'blink-cmp-copilot',
            score_offset = 100,
            async = true,
            transform_items = function(_, items)
              local CompletionItemKind = require('blink.cmp.types').CompletionItemKind
              local kind_idx = #CompletionItemKind + 1
              CompletionItemKind[kind_idx] = 'Copilot'
              for _, item in ipairs(items) do
                item.kind = kind_idx
              end
              return items
            end,
          },
        },
      },
    },

    appearance = {
      -- 'mono' (default) for 'Nerd Font Mono' or 'normal' for 'Nerd Font'
      -- Adjusts spacing to ensure icons are aligned
      use_nvim_cmp_as_default = true,
      nerd_font_variant = 'mono',
      kind_icons = {
        Copilot = '',
        Text = '󰉿',
        Method = '󰊕',
        Function = '󰊕',
        Constructor = '󰒓',

        Field = '󰜢',
        Variable = '󰆦',
        Property = '󰖷',

        Class = '󱡠',
        Interface = '󱡠',
        Struct = '󱡠',
        Module = '󰅩',

        Unit = '󰪚',
        Value = '󰦨',
        Enum = '󰦨',
        EnumMember = '󰦨',

        Keyword = '󰻾',
        Constant = '󰏿',

        Snippet = '󱄽',
        Color = '󰏘',
        File = '󰈔',
        Reference = '󰬲',
        Folder = '󰉋',
        Event = '󱐋',
        Operator = '󰪚',
        TypeParameter = '󰬛',
      },
    },

    completion = {
      -- By default, you may press `<c-space>` to show the documentation.
      -- Optionally, set `auto_show = true` to show the documentation after a delay.
      documentation = { auto_show = false, auto_show_delay_ms = 10 },
      ghost_text = { enabled = true },
      list = {
        selection = {
          preselect = false,
        },
      },
      menu = {
        draw = {
          columns = {
            { 'kind_icon', 'label', 'label_description', gap = 1 },
            { 'kind', 'source_name', gap = 1 },
          },
          components = {
            label_description = {
              width = { max = 50 },
            },
            source_name = {
              text = function(ctx)
                return '[' .. ctx.source_name .. ']'
              end,
            },
          },
        },
      },
    },

    sources = {
      default = { 'lsp', 'path', 'snippets', 'lazydev', 'buffer', 'nerdfont', 'emoji', 'copilot' },
      providers = {
        copilot = {
          name = 'copilot',
          module = 'blink-cmp-copilot',
          score_offset = 100,
          async = true,
        },
        snippets = {
          score_offset = 30,
        },
        lazydev = {
          name = 'LazyDev',
          module = 'lazydev.integrations.blink',
          score_offset = 50,
        },
        nerdfont = {
          module = 'blink-nerdfont',
          name = 'Nerd Fonts',
          -- score_offset = 15,
          opts = { insert = true },
        },
        emoji = {
          module = 'blink-emoji',
          name = 'Emoji',
          -- score_offset = 15,
          opts = { insert = true },
          should_show_items = function()
            return vim.tbl_contains({ 'gitcommit', 'markdown' }, vim.o.filetype)
          end,
        },
      },
      transform_items = function(_, items)
        return vim.tbl_filter(function(item)
          return not (item.kind == require('blink.cmp.types').CompletionItemKind.Snippet and item.source_name == 'LSP')
        end, items)
      end,
    },

    snippets = { preset = 'luasnip' },

    -- Blink.cmp includes an optional, recommended rust fuzzy matcher,
    -- which automatically downloads a prebuilt binary when enabled.
    --
    -- By default, we use the Lua implementation instead, but you may enable
    -- the rust implementation via `'prefer_rust_with_warning'`
    --
    -- See :h blink-cmp-config-fuzzy for more information
    fuzzy = { implementation = 'lua' },

    -- Shows a signature help window while you type arguments for a function
    signature = { enabled = true },
  },
}
--   { -- Autocompletion
--   'hrsh7th/nvim-cmp',
--   event = 'InsertEnter',
--   dependencies = {
--     -- Snippet Engine & its associated nvim-cmp source
--     {
--       'L3MON4D3/LuaSnip',
--       build = (function()
--         -- Build Step is needed for regex support in snippets.
--         -- This step is not supported in many windows environments.
--         -- Remove the below condition to re-enable on windows.
--         if vim.fn.has 'win32' == 1 or vim.fn.executable 'make' == 0 then
--           return
--         end
--         return 'make install_jsregexp'
--       end)(),
--       dependencies = {
--         -- `friendly-snippets` contains a variety of premade snippets.
--         --    See the README about individual language/framework/plugin snippets:
--         --    https://github.com/rafamadriz/friendly-snippets
--         -- {
--         --   'rafamadriz/friendly-snippets',
--         --   config = function()
--         --     require('luasnip.loaders.from_vscode').lazy_load()
--         --   end,
--         -- },
--       },
--     },
--     'saadparwaiz1/cmp_luasnip',
--
--     -- Adds other completion capabilities.
--     --  nvim-cmp does not ship with all sources by default. They are split
--     --  into multiple repos for maintenance purposes.
--     'hrsh7th/cmp-nvim-lsp',
--     'hrsh7th/cmp-path',
--     'onsails/lspkind-nvim',
--   },
--   config = function()
--     -- See `:help cmp`
--     local cmp = require 'cmp'
--     local luasnip = require 'luasnip'
--     local lspkind = require 'lspkind'
--
--     luasnip.config.setup {}
--
--     cmp.setup {
--       formatting = {
--         fields = { 'kind', 'abbr', 'menu' },
--         mode = 'symbol_text',
--         expandable_indicator = true,
--         format = lspkind.cmp_format {
--           symbol_map = { Copilot = '' },
--           maxwidth = {
--             menu = 50,
--             abbr = 50,
--           },
--           ellipsis_char = '...',
--           show_labelDetails = true,
--           menu = {
--             buffer = '[Buffer]',
--             luasnip = '[LuaSnip]',
--             nvim_lua = '[Lua]',
--             path = '[Path]',
--             emoji = '[Emoji]',
--           },
--         },
--       },
--       sorting = {
--         priority_weight = 2,
--         comparators = {
--           require('copilot_cmp.comparators').prioritize,
--
--           -- Below is the default comparitor list and order for nvim-cmp
--           cmp.config.compare.offset,
--           -- cmp.config.compare.scopes, --this is commented in nvim-cmp too
--           cmp.config.compare.exact,
--           cmp.config.compare.score,
--           cmp.config.compare.recently_used,
--           cmp.config.compare.locality,
--           cmp.config.compare.kind,
--           cmp.config.compare.sort_text,
--           cmp.config.compare.length,
--           cmp.config.compare.order,
--         },
--       },
--
--       snippet = {
--         expand = function(args)
--           luasnip.lsp_expand(args.body)
--         end,
--       },
--       completion = { completeopt = 'menu,menuone,noinsert' },
--
--       -- For an understanding of why these mappings were
--       -- chosen, you will need to read `:help ins-completion`
--       --
--       -- No, but seriously. Please read `:help ins-completion`, it is really good!
--       mapping = cmp.mapping.preset.insert {
--         -- Select the [n]ext item
--         ['<C-n>'] = cmp.mapping.select_next_item(),
--         -- Select the [p]revious item
--         ['<C-p>'] = cmp.mapping.select_prev_item(),
--
--         -- Scroll the documentation window [b]ack / [f]orward
--         ['<C-b>'] = cmp.mapping.scroll_docs(-4),
--         ['<C-f>'] = cmp.mapping.scroll_docs(4),
--
--         -- Accept ([y]es) the completion.
--         --  This will auto-import if your LSP supports it.
--         --  This will expand snippets if the LSP sent a snippet.
--         ['<C-y>'] = cmp.mapping.confirm { select = true },
--
--         -- If you prefer more traditional completion keymaps,
--         -- you can uncomment the following lines
--         --['<CR>'] = cmp.mapping.confirm { select = true },
--         --['<Tab>'] = cmp.mapping.select_next_item(),
--         --['<S-Tab>'] = cmp.mapping.select_prev_item(),
--
--         -- Manually trigger a completion from nvim-cmp.
--         --  Generally you don't need this, because nvim-cmp will display
--         --  completions whenever it has completion options available.
--         ['<C-Space>'] = cmp.mapping.complete {},
--
--         -- Think of <c-l> as moving to the right of your snippet expansion.
--         --  So if you have a snippet that's like:
--         --  function $name($args)
--         --    $body
--         --  end
--         --
--         -- <c-l> will move you to the right of each of the expansion locations.
--         -- <c-h> is similar, except moving you backwards.
--         ['<C-l>'] = cmp.mapping(function()
--           if luasnip.expand_or_locally_jumpable() then
--             luasnip.expand_or_jump()
--           end
--         end, { 'i', 's' }),
--         ['<C-h>'] = cmp.mapping(function()
--           if luasnip.locally_jumpable(-1) then
--             luasnip.jump(-1)
--           end
--         end, { 'i', 's' }),
--
--         -- For more advanced Luasnip keymaps (e.g. selecting choice nodes, expansion) see:
--         --    https://github.com/L3MON4D3/LuaSnip?tab=readme-ov-file#keymaps
--       },
--       sources = {
--         { name = 'copilot', group_index = 2 },
--         {
--           name = 'lazydev',
--           -- set group index to 0 to skip loading LuaLS completions as lazydev recommends it
--           group_index = 0,
--         },
--         { name = 'nvim_lsp' },
--         { name = 'luasnip' },
--         { name = 'path' },
--         { name = 'vim-dadbod-completion' },
--       },
--     }
--   end,
-- }
