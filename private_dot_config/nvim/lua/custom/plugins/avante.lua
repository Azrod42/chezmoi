return {
  'yetone/avante.nvim',
  -- dir = '~/Documents/avante.nvim',
  -- Never load at startup
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
  version = false,

  opts = {
    mode = 'agentic',
    provider = 'copilot',
    providers = {
      copilot = {
        endpoint = 'https://api.githubcopilot.com',
        model = 'gpt-5',
        allow_insecure = false,
        timeout = 30000,
        extra_request_body = {
          temperature = 0,
          max_tokens = 20480,
        },
      },
    },
    disabled_tools = {
      'list_files',
      'search_files',
      'read_file',
      'create_file',
      'rename_file',
      'delete_file',
      'create_dir',
      'rename_dir',
      'delete_dir',
      'bash',
    },
    model_override = {
      { name = 'gpt-4.1-mini', provider_name = 'openai', model = 'openai/gpt-4.1-mini' },
      { name = 'gpt-4.1-nano', provider_name = 'openai', model = 'openai/gpt-4.1-mini' },
      { name = 'Mistral 3', provider_name = 'openai', model = 'mistralai/mistral-medium-3' },
    },
    system_prompt = function()
      local hub = require('mcphub').get_hub_instance()
      if not hub then
        return 'Avante'
      end
      return hub:get_active_servers_prompt()
    end,
    custom_tools = function()
      return { require('mcphub.extensions.avante').mcp_tool() }
    end,
  },

  build = 'make',

  dependencies = {
    'nvim-treesitter/nvim-treesitter',
    'stevearc/dressing.nvim',
    'nvim-lua/plenary.nvim',
    'MunifTanjim/nui.nvim',
    'echasnovski/mini.pick',
    'nvim-telescope/telescope.nvim',
    'hrsh7th/nvim-cmp',
    'ibhagwan/fzf-lua',
    'nvim-tree/nvim-web-devicons',
    { 'zbirenbaum/copilot.lua', lazy = true }, -- only loads when Avante loads
    {
      'HakonHarnes/img-clip.nvim',
      lazy = true, -- remove VeryLazy to avoid startup load
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
      'MeanderingProgrammer/render-markdown.nvim',
      opts = { file_types = { 'markdown', 'Avante' } },
      ft = { 'markdown', 'Avante' }, -- loads only for markdown or Avante buffers
    },
  },
}
