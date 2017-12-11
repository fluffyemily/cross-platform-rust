/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit

protocol ToDoListItemsViewControllerDelegate {
    func itemCreated(item: Item)
    func itemUpdated(item: Item)
}

class ToDoListItemsTableViewController: UITableViewController {

    var items: [Item]!

    override func viewDidLoad() {
        super.viewDidLoad()

        self.items = ToodleLib.sharedInstance.list.allItems()

        self.title = "All Items"
        self.navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: UIBarButtonSystemItem.add, target: self, action: #selector(newItem))
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
    }

    // MARK: - Table view data source

    override func numberOfSections(in tableView: UITableView) -> Int {
        return 1
    }

    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return self.items.count
    }


    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "ItemCell") ?? UITableViewCell(style: .subtitle, reuseIdentifier: "ItemCell")
        let item = self.items[indexPath.row]
        cell.textLabel?.text = item.name
        cell.detailTextLabel?.text = item.dueDateAsString()

        return cell
    }

    override func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        let item = self.items[indexPath.row]
        let itemVC = ItemViewController(item: item)
        itemVC.delegate = self
        self.navigationController?.pushViewController(itemVC, animated: true)
    }

    @objc fileprivate func newItem() {
        let itemVC = ItemViewController()
        itemVC.delegate = self
        let navController = UINavigationController(rootViewController: itemVC)
        self.present(navController, animated: true, completion: nil)
    }

}

extension ToDoListItemsTableViewController: ToDoListItemsViewControllerDelegate {
    func itemCreated(item: Item) {
        self.items.append(item)
        self.tableView.reloadData()
    }

    func itemUpdated(item: Item) {
        guard let index = self.items.index(where: { i in item.uuid == i.uuid }) else {
            return itemCreated(item: item)
        }
        self.items[index] = item
        self.items = ToodleLib.sharedInstance.list.allItems()
        self.tableView.reloadData()
    }
}
