return {
  'yetone/avante.nvim',
  -- dir = '~/documents/avante.nvim',
  -- never load at startup
  build = 'make',
  version = false,
  cmd = { 'AvanteAsk', 'AvanteChat', 'AvanteToggle', 'AvanteModels', 'AvanteFocus', 'AvanteRefresh', 'AvanteStop' },
  keys = {
    {
      '<leader>aa',
      function()
        vim.cmd 'AvanteAsk'
      end,
      desc = 'Avante: Ask',
    },
    {
      '<leader>at',
      function()
        vim.cmd 'AvanteToggle'
      end,
      desc = 'Avante: Toggle',
    },
    {
      '<leader>ar',
      function()
        vim.cmd 'AvanteRefresh'
      end,
      desc = 'Avante: Refresh',
    },
  },

  opts = {
    mode = 'agentic',
    provider = 'codex',
    acp_providers = {
      ['codex'] = {
        command = 'npx',
        args = { '-y', '-g', '@zed-industries/codex-acp' },
        env = {
          node_no_warnings = '1',
          home = os.getenv 'home',
          path = os.getenv 'path',
        },
      },
    },
    providers = {
      openai = {
        endpoint = 'https://openrouter.ai/api/v1',
        model = 'openai/gpt-5.4-mini:nitro',
        api_key_name = 'openrouter_api_key_avante',
        extra_request_body = {
          temperature = 1,
          max_tokens = 20480,
        },
      },
    },
    -- disabled_tools = {
    --   'list_files',
    --   'search_files',
    --   'read_file',
    --   'create_file',
    --   'rename_file',
    --   'delete_file',
    --   'create_dir',
    --   'rename_dir',
    --   'delete_dir',
    --   'bash',
    -- },
    model_override = {
      -- { name = 'gpt-4.1-mini', provider_name = 'openai', model = 'openai/gpt-4.1-mini' },
      -- { name = 'gpt-4.1-nano', provider_name = 'openai', model = 'openai/gpt-4.1-mini' },
      -- { name = 'mistral 3', provider_name = 'openai', model = 'mistralai/mistral-medium-3' },
    },
    -- system_prompt = function()
    --   local hub = require('mcphub').get_hub_instance()
    --   if not hub then
    --     return 'avante'
    --   end
    --   return hub:get_active_servers_prompt()
    -- end,
    -- custom_tools = function()
    --   return { require('mcphub.extensions.avante').mcp_tool() }
    -- end,
  },

  dependencies = {
    'nvim-lua/plenary.nvim',
    'muniftanjim/nui.nvim',

    'echasnovski/mini.pick',
    'nvim-telescope/telescope.nvim',
    'hrsh7th/nvim-cmp',
    'nvim-treesitter/nvim-treesitter',
    'folke/snacks.nvim',
    'stevearc/dressing.nvim',
    'ibhagwan/fzf-lua',
    'nvim-tree/nvim-web-devicons',
    -- { 'zbirenbaum/copilot.lua', lazy = true }, -- only loads when avante loads
    {
      'hakonharnes/img-clip.nvim',
      lazy = true, -- remove verylazy to avoid startup load
      opts = {
        default = {
          embed_image_as_base64 = false,
          prompt_for_file_name = false,
          drag_and_drop = { insert_mode = true },
          use_absolute_path = true,
        },
      },
    },
    {
      'meanderingprogrammer/render-markdown.nvim',
      opts = { file_types = { 'markdown', 'avante' } },
      ft = { 'markdown', 'avante' }, -- loads only for markdown or avante buffers
    },
  },
}
