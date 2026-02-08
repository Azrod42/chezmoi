return {
  {
    'windwp/nvim-ts-autotag',
    ft = { 'javascript', 'javascriptreact', 'typescript', 'typescriptreact', 'html', 'markdown' },
    config = function()
      require('nvim-ts-autotag').setup()
    end,
  },
  {
    'pmizio/typescript-tools.nvim',
    dependencies = { 'nvim-lua/plenary.nvim', 'neovim/nvim-lspconfig' },
    opts = {},
  },
}
