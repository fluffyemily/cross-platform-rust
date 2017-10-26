/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit

protocol ToDoListItemsViewControllerDelegate {
    func itemSaveSuccess(item: Item)
}

class ToDoListItemsTableViewController: UITableViewController {

    var category: Category

    init(category: Category) {
        self.category = category
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder aDecoder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        self.title = category.name
        self.navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: UIBarButtonSystemItem.add, target: self, action: #selector(newItem))
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

    // MARK: - Table view data source

    override func numberOfSections(in tableView: UITableView) -> Int {
        // #warning Incomplete implementation, return the number of sections
        return 1
    }

    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        // #warning Incomplete implementation, return the number of rows
        return category.items.count
    }


    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "ItemCell") ?? UITableViewCell(style: .subtitle, reuseIdentifier: "ItemCell")
        let item = self.category.items[indexPath.row]
        cell.textLabel?.text = item.description
        cell.detailTextLabel?.text = item.dueDateAsString()
        // Configure the cell...

        return cell
    }

    override func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        let item = self.category.items[indexPath.row]
        let itemVC = ItemViewController(item: item, category: self.category)
        itemVC.delegate = self
        self.navigationController?.pushViewController(itemVC, animated: true)
    }

    @objc fileprivate func newItem() {
        let itemVC = ItemViewController(category: self.category)
        itemVC.delegate = self
        let navController = UINavigationController(rootViewController: itemVC)
        self.present(navController, animated: true, completion: nil)
    }

}

extension ToDoListItemsTableViewController: ToDoListItemsViewControllerDelegate {
    func itemSaveSuccess(item: Item) {
        if !self.category.items.contains(where: {  $0.id == item.id }) {
            self.category.add_item(item: item)
        }
        self.tableView.reloadData()
    }
}
